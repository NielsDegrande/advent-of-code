use std::collections::HashSet;
use std::fs;
use std::path::Path;

// Choose the mode: Test or Real.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

fn parse_data(contents: &String) -> Vec<Vec<u8>> {
    let mut rows: Vec<Vec<u8>> = Vec::new();
    // Parse line by line.
    for line in contents.lines() {
        let heights: Vec<u8> = line
            .split("")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        rows.push(heights);
    }
    rows
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn add_number_of_visible_trees_for_row_direction(
    data: &Vec<Vec<u8>>,
    reverse: bool,
) -> HashSet<(usize, usize, u8)> {
    let mut counted_trees: HashSet<(usize, usize, u8)> = HashSet::new();
    let max_dim_1 = data.len();
    let max_dim_2 = data[0].len();

    for row in 1..max_dim_1 - 1 {
        let start_tree_height: u8 = if reverse {
            data[row][data[0].len() - 1]
        } else {
            data[row][0]
        };
        let mut highest_tree_so_far: u8 = start_tree_height;

        // NOTE: More performant (without allocating) option with itertools::Either exists.
        let range_: Vec<usize> = if reverse {
            (1..max_dim_2 - 1).rev().collect()
        } else {
            (1..max_dim_2 - 1).collect()
        };
        for tree in range_ {
            let tree_height: u8 = data[row][tree];
            if tree_height > highest_tree_so_far {
                highest_tree_so_far = tree_height;
                counted_trees.insert((row, tree, tree_height));
            }
        }
    }

    counted_trees
}

fn solve_part_1(data: &Vec<Vec<u8>>) -> u32 {
    let forest_len_y: u32 = data.len() as u32;
    let forest_len_x: u32 = data[0].len() as u32;
    let mut counted_trees: HashSet<(usize, usize, u8)> = HashSet::new();

    // Keep track of solution.
    // Initialize to outer trees. NOTE: Avoid double counting the edges.
    let mut solution: u32 = forest_len_x * 2 + (forest_len_y - 2) * 2;

    // Parse row by row going from the left.
    let additional_trees = add_number_of_visible_trees_for_row_direction(data, false);
    counted_trees.extend(additional_trees);

    // Parse row by row going from the right.
    let additional_trees = add_number_of_visible_trees_for_row_direction(data, true);
    counted_trees.extend(additional_trees);

    // Transpose the input data.
    let transposed_data: Vec<Vec<u8>> = transpose(data);

    // Parse column by column going from the top.
    let additional_trees = add_number_of_visible_trees_for_row_direction(&transposed_data, false);
    // Swap coordinates again given transpose.
    counted_trees.extend(
        additional_trees
            .iter()
            .map(|tree: &(usize, usize, u8)| (tree.1, tree.0, tree.2)),
    );

    // Parse column by column going from the bottom.
    let additional_trees = add_number_of_visible_trees_for_row_direction(&transposed_data, true);
    // Swap coordinates again given transpose.
    counted_trees.extend(
        additional_trees
            .iter()
            .map(|tree: &(usize, usize, u8)| (tree.1, tree.0, tree.2)),
    );

    // Add unique visible trees in the center.
    solution += counted_trees.len() as u32;

    solution
}

fn compute_viewing_distance(row: &Vec<u8>, current_index: usize, increase: bool) -> u32 {
    let mut viewing_distance: u32 = 0;
    let tree_height: u8 = row[current_index];
    let mut loop_index: usize = current_index;

    let end_index = if increase { row.len() - 1 } else { usize::MIN };
    while if increase {
        end_index > loop_index
    } else {
        loop_index > end_index
    } {
        if increase {
            loop_index += 1
        } else {
            loop_index -= 1
        };
        if tree_height > row[loop_index] {
            viewing_distance += 1;
        } else if tree_height <= row[loop_index] {
            viewing_distance += 1;
            break;
        }
    }

    viewing_distance
}

fn solve_part_2(data: &Vec<Vec<u8>>) -> u32 {
    // Keep track of solution.
    let mut solution = 0;

    // Transpose the input data.
    let transposed_data: Vec<Vec<u8>> = transpose(data);

    // Loop over the rows.
    for row_index in 0..data.len() {
        for column_index in 0..data[0].len() {
            let left_viewing_distance =
                compute_viewing_distance(&data[row_index], column_index, false);
            let right_viewing_distance =
                compute_viewing_distance(&data[row_index], column_index, true);
            let top_viewing_distance =
                compute_viewing_distance(&transposed_data[column_index], row_index, false);
            let bottom_viewing_distance =
                compute_viewing_distance(&transposed_data[column_index], row_index, true);

            // Compute scenic score.
            let current_scenic_score: u32 = left_viewing_distance
                * right_viewing_distance
                * top_viewing_distance
                * bottom_viewing_distance;

            // Update solution when a higher scenic score is found.
            if current_scenic_score > solution {
                solution = current_scenic_score;
            }
        }
    }

    solution
}

fn main() {
    // Initialize problem.
    let contents: String;
    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            contents = read_file("data/test.txt");
        }
        MODE::Real => {
            // Read input file.
            contents = read_file("data/input.txt");
        }
    }
    let data: Vec<Vec<u8>> = parse_data(&contents);

    // Part 1.
    let part_1_score = solve_part_1(&data);
    println!("Part 1 - Solution: {}", part_1_score);

    // Part 2.
    let part_2_score = solve_part_2(&data);
    println!("Part 2 - Solution: {}", part_2_score);
}
