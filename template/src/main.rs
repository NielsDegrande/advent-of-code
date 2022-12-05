// Hide dead code and unused variables warning while developing.
// TODO: Remove before committing.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::path::Path;

// Choose the mode: Real or Test.
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

fn solve_part_1(contents: &String) -> u32 {
    // Keep track of solution.
    let mut solution = 0;

    // Parse line by line.
    for line in contents.lines() {
        println!("{}", line);
        solution += 1
    }
    solution
}

fn solve_part_2(contents: &String) -> u32 {
    // Keep track of solution.
    let mut solution = 0;

    // Parse line by line.
    for line in contents.lines() {
        println!("{}", line);
        solution += 1
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

    // Part 1.
    let part_1_score = solve_part_1(&contents);
    println!("Part 1 - Solution: {}", part_1_score);

    // Part 2.
    let part_2_score = solve_part_2(&contents);
    println!("Part 2 - Solution: {}", part_2_score);
}
