// Choose the mode: Test or Real.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

const RADIX: u32 = 10;

fn reverse_string(string: &str) -> String {
    string.chars().rev().collect()
}

fn find_number(input: &str) -> Option<(usize, u32)> {
    input
        .chars()
        .enumerate()
        .find_map(|(index, c)| c.to_digit(RADIX).map(|number| (index, number)))
}

fn replace_numbers(line: &str) -> String {
    line.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

fn solve_part_1(input: &str) -> u32 {
    // Keep track of the solution.
    let mut solution = 0;

    // Parse line by line.
    for line in input.lines() {
        let left_digit = find_number(line).unwrap().1 * RADIX;
        let right_digit = find_number(&reverse_string(line)).unwrap().1;
        solution += left_digit + right_digit;
    }
    solution
}

fn solve_part_2(input: &str) -> u32 {
    // Keep track of the solution.
    let mut solution = 0;

    // Parse line by line.
    for line in input.lines() {
        let replaced_line = replace_numbers(line);
        let left_digit = find_number(&replaced_line).unwrap().1 * RADIX;
        let right_digit = find_number(&reverse_string(&replaced_line)).unwrap().1;
        solution += left_digit + right_digit;
    }
    solution
}

fn main() {
    // Initialize problem.
    let mut input: &str;
    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            input = include_str!("../data/test1.txt");
        }
        MODE::Real => {
            // Read input file.
            input = include_str!("../data/input.txt");
        }
    }

    // Part 1.
    let part_1_score = solve_part_1(input);
    println!("Part 1 - Solution: {}", part_1_score);

    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            input = include_str!("../data/test2.txt");
        }
        MODE::Real => {
            // Read input file.
            input = include_str!("../data/input.txt");
        }
    }

    // Part 2.
    let part_2_score = solve_part_2(input);
    println!("Part 2 - Solution: {}", part_2_score);
}
