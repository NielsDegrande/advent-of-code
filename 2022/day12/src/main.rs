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
    fs::read_to_string(file_path).expect("Should have been able to read the file.")
}

fn parse_data(contents: &String) -> (Vec<Vec<char>>, (i8, i8), (i8, i8)) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_position: (i8, i8) = (0, 0);
    let mut end_position: (i8, i8) = (0, 0);

    // Find start and end.
    for (row_index, line) in contents.lines().enumerate() {
        for (column_index, character) in line.chars().enumerate() {
            if character == 'S' {
                start_position = (row_index as i8, column_index as i8);
            } else if character == 'E' {
                end_position = (row_index as i8, column_index as i8);
            }
        }
        // Split and add row to map.
        map.push(line.chars().collect());
    }

    println!("Start: {:?}", start_position);
    println!("End: {:?}", end_position);

    (map, start_position, end_position)
}

fn find_valid_adjacent_paths(
    map: &Vec<Vec<char>>,
    tail_position: &(i8, i8, char),
) -> Vec<(i8, i8, char)> {
    let possible_positions = vec![
        (tail_position.0, tail_position.1 - 1), // Left.
        (tail_position.0, tail_position.1 + 1), // Right.
        (tail_position.0 - 1, tail_position.1), // Up.
        (tail_position.0 + 1, tail_position.1), // Down.
    ];

    let mut valid_adjacent_paths: Vec<(i8, i8, char)> = Vec::new();
    for position in possible_positions.iter() {
        // Validate bounds.
        if position.0 >= 0
            && position.0 < map.len() as i8
            && position.1 >= 0
            && position.1 < map[0].len() as i8
        {
            let character = map[position.0 as usize][position.1 as usize];
            // Treat 'S' and 'E' as 'a' and 'z' respectively.
            let destination = if character == 'S' { 'a' } else { character } as i32;
            let current = if tail_position.2 == 'E' {
                'z'
            } else {
                tail_position.2
            } as i32;
            // Validate you do not climb more than 1 height unit at a time.
            // Given that we go backwards from goal, this means descend height not more than -1.
            if destination - current >= -1 {
                valid_adjacent_paths.push((position.0, position.1, character))
            };
        }
    }
    valid_adjacent_paths
}

fn find_path(
    map: &Vec<Vec<char>>,
    start_position: &(i8, i8),
    start_char: char,
    end_char: char,
) -> Vec<(i8, i8, char)> {
    // Keep track of paths under consideration: this holds Dijkstra's shortest path tree.
    let mut paths: Vec<Vec<(i8, i8, char)>> = Vec::new();

    // Insert start position.
    let start_position_with_char = (start_position.0, start_position.1, start_char);
    let start_path = vec![start_position_with_char];
    paths.push(start_path);

    // Keep track of visited positions: the seen set.
    let mut visited_positions: HashSet<(i8, i8, char)> = HashSet::new();
    visited_positions.insert(start_position_with_char);

    // Implement a modified shortest path algorithm.
    loop {
        let mut updated_paths: Vec<Vec<(i8, i8, char)>> = Vec::new();
        for path in &paths {
            // Find current tail position of the path.
            let tail_position = path[path.len() - 1];
            let next_positions = find_valid_adjacent_paths(&map, &tail_position);
            for next_position in next_positions {
                // Do not go back to earlier visited positions. NOTE: This check also avoid circles.
                if visited_positions.contains(&next_position) {
                    continue;
                }
                // Add next position to visited positions.
                visited_positions.insert(next_position);

                // Create new path until next position. To be added to the tree.
                let mut updated_path = path.clone();
                updated_path.push(next_position);
                
                // End looping once a path until end_char has been found.
                if next_position.2 == end_char {
                    return updated_path;
                }
                
                // If not end_char, add the path to the list of paths that will be explored further.
                updated_paths.push(updated_path);
            }
        }
        if updated_paths.len() == 0 {
            println!("WARNING: Impossible to find path from start to finish!");
            return vec![];
        }
        paths = updated_paths;
    }
}

fn visualize_solution(map: &Vec<Vec<char>>, path: Vec<(i8, i8)>) {
    for row_index in 0..map.len() {
        let mut print_line: String = String::new();
        for column_index in 0..map[0].len() {
            if path.contains(&(row_index as i8, column_index as i8)) {
                let character = map[row_index][column_index];
                print_line.push(character);
            } else {
                print_line.push('.');
            }
        }
        println!("{}", print_line);
    }
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

    // Parse content into useful data.
    let (map, _, end_position) = parse_data(&contents);

    // Part 1.
    // Swap start and end to search from the back.
    let part_1_path = find_path(&map, &end_position, 'E', 'S');
    if part_1_path.len() > 0 {
        println!("Part 1 - Solution: {}", part_1_path.len() - 1);
        // Visualize output.
        let part_1_clean_path: Vec<(i8, i8)> = part_1_path.iter().map(|x| (x.0, x.1)).collect();
        visualize_solution(&map, part_1_clean_path);
    }

    // Part 2.
    // Swap start and end to search from the back.
    let part_2_path = find_path(&map, &end_position, 'E', 'a');
    if part_2_path.len() > 0 {
        println!("Part 2 - Solution: {}", part_2_path.len() - 1);
        // Visualize output.
        let part_2_clean_path: Vec<(i8, i8)> = part_2_path.iter().map(|x| (x.0, x.1)).collect();
        visualize_solution(&map, part_2_clean_path);
    }
}
