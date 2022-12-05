use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Choose the mode: Real or Test.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Real,
    Test,
}

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn parse_cargo_data(
    contents: &String,
    number_of_stacks: usize,
    cargo_height: usize,
) -> HashMap<usize, Vec<String>> {
    // Select cargo data.
    let cargo_data: Vec<&str> = contents.lines().take(cargo_height).collect();

    // Set up data structure.
    let mut cargo_map = HashMap::new();
    for stack in 1..number_of_stacks + 1 {
        cargo_map.insert(stack, vec![]);
    }

    // Parse cargo data layer by layer.
    for layer in (0..cargo_height).rev() {
        // Extract data.
        let layer_data: &str = cargo_data[layer];
        for stack in 1..number_of_stacks + 1 {
            let parsed_crate: &str = &layer_data[4 * (stack - 1)..4 * (stack - 1) + 3];
            let stack_in_map = cargo_map.get_mut(&stack).unwrap();
            let trimmed_crate = remove_whitespace(parsed_crate);
            if trimmed_crate != "" {
                stack_in_map.push(trimmed_crate);
            }
        }
    }

    cargo_map
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let instruction_parts: Vec<&str> = instruction.split_whitespace().collect();
    let number_of_crates: usize = instruction_parts[1].parse().unwrap();
    let from_stack: usize = instruction_parts[3].parse().unwrap();
    let to_stack: usize = instruction_parts[5].parse().unwrap();

    (number_of_crates, from_stack, to_stack)
}

fn format_solution(cargo_map: &HashMap<usize, Vec<String>>, number_of_stacks: usize) -> String {
    let mut solution: String = String::from("");
    for stack in 1..number_of_stacks + 1 {
        let stack_in_map = cargo_map.get(&stack).unwrap();
        let top_crate = stack_in_map.last().unwrap();
        solution = solution + &top_crate.replace("[", "").replace("]", "");
    }

    solution
}

fn solve_part_1(
    contents: &String,
    cargo_map: &mut HashMap<usize, Vec<String>>,
    number_of_stacks: usize,
    cargo_height: usize,
) -> String {
    // Select instruction data.
    let instructions: Vec<&str> = contents.lines().skip(cargo_height + 2).collect();

    // Parse instruction data line by line.
    for instruction in instructions {
        let (number_of_crates, from_stack, to_stack) = parse_instruction(instruction);

        // Move crates around.
        for _ in 0..number_of_crates {
            let from_stack_in_map = cargo_map.get_mut(&from_stack).unwrap();
            let crate_ = from_stack_in_map.pop().unwrap();

            let to_stack_in_map = cargo_map.get_mut(&to_stack).unwrap();
            to_stack_in_map.push(crate_);
        }
    }

    // Format solution.
    format_solution(&cargo_map, number_of_stacks)
}

fn solve_part_2(
    contents: &String,
    cargo_map: &mut HashMap<usize, Vec<String>>,
    number_of_stacks: usize,
    cargo_height: usize,
) -> String {
    // Select instruction data.
    let instructions: Vec<&str> = contents.lines().skip(cargo_height + 2).collect();

    // Parse instruction data line by line.
    for instruction in instructions {
        let (number_of_crates, from_stack, to_stack) = parse_instruction(instruction);

        // Move crates around.
        let from_stack_in_map = cargo_map.get_mut(&from_stack).unwrap();
        let crates = from_stack_in_map.split_off(from_stack_in_map.len() - number_of_crates);
        let to_stack_in_map = cargo_map.get_mut(&to_stack).unwrap();
        to_stack_in_map.extend(crates);
    }

    // Format solution.
    format_solution(&cargo_map, number_of_stacks)
}

fn main() {
    // Initialize problem.
    let contents: String;
    let number_of_stacks: usize;
    let cargo_height: usize;
    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            contents = read_file("data/test.txt");
            number_of_stacks = 3;
            cargo_height = 3;
        }
        MODE::Real => {
            // Read input file.
            contents = read_file("data/input.txt");
            number_of_stacks = 9;
            cargo_height = 8;
        }
    }

    // Parse cargo data.
    let cargo_map: HashMap<usize, Vec<String>> =
        parse_cargo_data(&contents, number_of_stacks, cargo_height);

    // Part 1.
    let part_1_solution = solve_part_1(
        &contents,
        &mut cargo_map.clone(),
        number_of_stacks,
        cargo_height,
    );
    println!("Part 1 - Solution: {}", part_1_solution);

    // Part 2.
    let part_2_solution = solve_part_2(
        &contents,
        &mut cargo_map.clone(),
        number_of_stacks,
        cargo_height,
    );
    println!("Part 2 - Solution: {}", part_2_solution);
}
