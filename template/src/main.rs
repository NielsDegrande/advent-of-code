// Hide dead code and unused variables warning while developing.
// TODO: Remove before committing.
#![allow(dead_code)]
#![allow(unused_variables)]

// Choose the mode: Test or Real.
const CHOSEN_MODE: MODE = MODE::Test;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

fn solve_part_1(input: &str) -> u32 {
    // Keep track of the solution.
    let mut solution = 0;

    // Parse line by line.
    for line in input.lines() {
        println!("{}", line);
        solution += 1;
    }
    solution
}

fn solve_part_2(input: &str) -> u32 {
    // Keep track of the solution.
    let mut solution = 0;

    // Parse line by line.
    for line in input.lines() {
        println!("{}", line);
        solution += 1;
    }
    solution
}

fn main() {
    // Initialize problem.
    let input: &str;
    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            input = include_str!("../data/test.txt");
        }
        MODE::Real => {
            // Read input file.
            input = include_str!("../data/input.txt");
        }
    }

    // Part 1.
    let part_1_score = solve_part_1(input);
    println!("Part 1 - Solution: {}", part_1_score);

    // Part 2.
    let part_2_score = solve_part_2(input);
    println!("Part 2 - Solution: {}", part_2_score);
}
