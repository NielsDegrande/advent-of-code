use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

mod helpers;

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

fn move_position(position: &mut (i32, i32), direction: &str) {
    match direction {
        "L" => position.0 = position.0 - 1,
        "R" => position.0 = position.0 + 1,
        "U" => position.1 = position.1 + 1,
        "D" => position.1 = position.1 - 1,
        _ => {
            println!("ERROR: Illegal move!")
        }
    };
}

fn move_tail(leader: &(i32, i32), follower: &mut (i32, i32)) {
    if !helpers::is_adjacent(leader, follower) {
        // Higher row.
        if leader.1 > follower.1 {
            move_position(follower, "U");
        // Lower row.
        } else if leader.1 < follower.1 {
            move_position(follower, "D");
        } else {
        }
        // More right column.
        if leader.0 > follower.0 {
            move_position(follower, "R");
        // More left column.
        } else if leader.0 < follower.0 {
            move_position(follower, "L");
        } else {
        }
    }
}

fn solve_part_1(contents: &String) -> usize {
    // Keep track of solution.
    let start_position: (i32, i32) = (0, 0);
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head_position = start_position.clone();
    let mut tail_position = start_position.clone();

    // Parse line by line.
    for line in contents.lines() {
        let instructions: Vec<&str> = line.split_whitespace().collect();
        let direction: &str = instructions[0];
        let steps: i32 = instructions[1].parse().unwrap();
        for _ in 0..steps {
            move_position(&mut head_position, direction);
            move_tail(&head_position, &mut tail_position);

            // Track all tail positions.
            visited_positions.insert(tail_position);
        }
    }

    visited_positions.len()
}

fn solve_part_2(contents: &String) -> usize {
    // // Keep track of solution.
    let start_position: (i32, i32) = (0, 0);
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head_position = start_position.clone();
    // Set up all possible tail positions.
    let mut tail_positions: HashMap<u8, (i32, i32)> = HashMap::new();
    for i in 1..10 {
        tail_positions.insert(i, start_position.clone());
    }

    // Parse line by line.
    for line in contents.lines() {
        let instructions: Vec<&str> = line.split_whitespace().collect();
        let direction: &str = instructions[0];
        let steps: i32 = instructions[1].parse().unwrap();
        for _ in 0..steps {
            move_position(&mut head_position, direction);
            for i in 1..10 {
                // Get previous knot.
                let leader: (i32, i32);
                if i == 1 {
                    leader = head_position;
                } else {
                    let previous = tail_positions.get(&(i - 1)).unwrap().clone();
                    leader = previous;
                }

                // Get the follower knot.
                let follower = tail_positions.get_mut(&i).unwrap();

                // Update the follower knot given the leader knot.
                move_tail(&leader, follower);

                // Track all tail positions.
                if i == 9 {
                    visited_positions.insert(follower.clone());
                }
            }
        }
    }

    visited_positions.len()
}

fn main() {
    // Initialize problem.
    let contents: String;
    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            contents = read_file("data/test.txt");
            // contents = read_file("data/test2.txt");
        }
        MODE::Real => {
            // Read input file.
            contents = read_file("data/input.txt");
        }
    }

    // Part 1.
    let part_1_score = solve_part_1(&contents);
    println!("Part 1 - Solution: {}", part_1_score);

    // Part 2.
    let part_2_score = solve_part_2(&contents);
    println!("Part 2 - Solution: {}", part_2_score);
}
