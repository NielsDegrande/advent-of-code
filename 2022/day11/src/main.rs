use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Choose the mode: Test or Real.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

// Magic numbers.
const NUMBER_OF_ROUNDS: usize = 10000;
const WORRY_DIVISOR: u64 = 1;

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    fs::read_to_string(file_path).expect("Should have been able to read the file.")
}

fn parse_data(
    contents: &String,
) -> (
    HashMap<u64, Vec<u64>>,
    HashMap<u64, Vec<String>>,
    HashMap<u64, HashMap<&str, u64>>,
) {
    // Define structures to hold parsed data.
    let mut monkey_item_map: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut monkey_operations: HashMap<u64, Vec<String>> = HashMap::new();
    let mut monkey_tests: HashMap<u64, HashMap<&str, u64>> = HashMap::new();

    // Loop over the lines and populate structures.
    let mut monkey_id: u64 = 0;
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line.contains("Monkey") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            monkey_id = parts[parts.len() - 1].replace(":", "").parse().unwrap();
        } else if line.contains("Starting items") {
            let items: Vec<u64> = line
                .replace("  Starting items: ", "")
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            monkey_item_map.insert(monkey_id, items);
        } else if line.contains("Operation") {
            let cleaned_line: String = line.replace("  Operation: new = ", "");
            let operations: Vec<String> = cleaned_line
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();
            monkey_operations.insert(monkey_id, operations);
        } else if line.contains("Test") {
            let test: u64 = line.replace("  Test: divisible by ", "").parse().unwrap();
            let if_true: u64 = lines
                .next()
                .unwrap()
                .replace("    If true: throw to monkey ", "")
                .parse()
                .unwrap();
            let if_false: u64 = lines
                .next()
                .unwrap()
                .replace("    If false: throw to monkey ", "")
                .parse()
                .unwrap();
            let test_specs =
                HashMap::from([("test", test), ("true", if_true), ("false", if_false)]);
            monkey_tests.insert(monkey_id, test_specs);
        } else if line == "" {
            // Skip.
        } else {
            println!("WARNING: Unexpected line!");
        }
    }
    (monkey_item_map, monkey_operations, monkey_tests)
}

fn solve_part_1(
    monkey_item_map: &mut HashMap<u64, Vec<u64>>,
    monkey_operations: HashMap<u64, Vec<String>>,
    monkey_tests: HashMap<u64, HashMap<&str, u64>>,
) -> u64 {
    // Keep track of the number of inspections a monkey performed.
    let mut monkey_inspections: HashMap<u64, u64> = HashMap::new();
    // Initialize inspection count at 0.
    for monkey in 0..monkey_item_map.len() {
        monkey_inspections.insert(monkey as u64, 0);
    }

    // Compute least common multiple to keep worry levels from overflowing.
    let mut least_common_multiple = 1;
    for monkey in 0..monkey_tests.len() {
        let test = monkey_tests.get(&(monkey as u64)).unwrap();
        let denominator = test.get("test").unwrap();
        least_common_multiple *= *denominator;
    }

    // Loop over the rounds.
    for _ in 0..NUMBER_OF_ROUNDS {
        // Every monkey plays once during a round.
        for monkey in 0..monkey_item_map.len() {
            // println!("Monkey {}:", monkey);
            let monkey_id: &u64 = &(monkey as u64);
            // Inspect all items a monkey holds.
            let items = monkey_item_map.get(monkey_id).unwrap().clone();
            for item in items.iter() {
                // Increase monkey inspection count.
                let monkey_inspection_count = monkey_inspections.get(&monkey_id).unwrap();
                monkey_inspections.insert(*monkey_id, monkey_inspection_count + 1);
                // println!("  Monkey inspects an item with a worry level of {}.", item);
                let operation = monkey_operations.get(monkey_id).unwrap();
                // Parse left hand and right hand information.
                if operation[0] != "old" {
                    println!("ERROR: Unexpected input!");
                }
                let right_element = if operation[2] == "old" {
                    *item
                } else {
                    operation[2].parse().unwrap()
                };
                // Apply operator.
                let mut worry_level: u64 = match &operation[1][..] {
                    "*" => {
                        let result = item * right_element;
                        // println! {"    Worry level is multiplied by {} to {}.", right_element, result};
                        result
                    }
                    "+" => {
                        let result = item + right_element;
                        // println! {"    Worry level increases by {} to {}.", right_element, result};
                        result
                    }
                    _ => {
                        println! {"ERROR: Unexpected operator!"};
                        0
                    }
                };
                // Apply test.
                let test = monkey_tests.get(monkey_id).unwrap();
                worry_level /= WORRY_DIVISOR;
                // println! {"    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level};
                let denominator = test.get("test").unwrap();
                let throw_to: &u64 = if worry_level % denominator == 0 {
                    // println!("    Current worry level is divisible by {}.", denominator);
                    test.get("true").unwrap()
                } else {
                    // println!("    Current worry level is not divisible by {}.", denominator);
                    test.get("false").unwrap()
                };
                let throw_to_item_map = monkey_item_map.get_mut(throw_to).unwrap();
                // println!("    Item with worry level {} is thrown to monkey {}.", worry_level, throw_to);
                worry_level = if WORRY_DIVISOR == 1 {
                    worry_level % least_common_multiple
                } else {
                    worry_level
                };
                throw_to_item_map.push(worry_level);
            }
            monkey_item_map.insert(*monkey_id, Vec::from([]));
        }
        // println!("Round {}: {:?}", round, monkey_item_map);
    }

    // Use inspection count to find the solution.
    let mut inspections: Vec<u64> = monkey_inspections.values().cloned().collect();

    // Sort inspections to find top 2.
    inspections.sort();
    inspections.reverse();

    // Multiply to get to final answer.
    inspections[0] * inspections[1]
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

    let (monkey_item_map, monkey_operations, monkey_tests) = parse_data(&contents);

    let score = solve_part_1(
        &mut monkey_item_map.clone(),
        monkey_operations,
        monkey_tests,
    );
    println!("Solution: {}", score);
}
