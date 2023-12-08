use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

fn parse_data(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let first_line = lines
        .next()
        .expect("First line is missing.")
        .chars()
        .collect();

    let hashmap: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|line: &str| {
            let (node, elements) = line.split_once(" = ").expect("Invalid line.");
            let (element1, element2) = elements
                .trim_matches(|p| p == '(' || p == ')')
                .split_once(", ")
                .expect("Invalid line.");
            (node.trim(), (element1, element2))
        })
        .collect();
    (first_line, hashmap)
}

fn solve_part_1(input: &str) -> u64 {
    let (instructions, network) = parse_data(input);
    let mut current_node = "AAA";
    let mut steps: u64 = 0;

    for instruction in instructions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }

        steps += 1;
        let &(element1, element2) = network.get(current_node).expect("Invalid node.");
        current_node = if *instruction == 'L' {
            element1
        } else {
            element2
        };
    }

    steps
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &Vec<u64>) -> u64 {
    numbers
        .iter()
        .copied()
        .reduce(|a, b| lcm(a, b))
        .unwrap_or(1)
}

fn solve_part_2(input: &str) -> u64 {
    let (instructions, network) = parse_data(input);
    let starting_nodes = network
        .keys()
        .filter(|&node| node.ends_with("A"))
        .copied()
        .collect::<Vec<&str>>();

    let mut steps = Vec::new();
    starting_nodes.iter().for_each(|&node| {
        let mut current_node = node;
        let mut steps_for_node: u64 = 0;
        for instruction in instructions.iter().cycle() {
            if current_node.ends_with('Z') {
                break;
            }

            steps_for_node += 1;
            let &(element1, element2) = network.get(current_node).expect("Invalid node.");
            current_node = if *instruction == 'L' {
                element1
            } else {
                element2
            };
        }
        steps.push(steps_for_node);
    });
    lcm_of_vec(&steps)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modes = [Mode::TEST, Mode::REAL];

    for mode in modes.iter() {
        let file_name = match mode {
            Mode::TEST => "test",
            Mode::REAL => "input",
        };
        let file_path = format!("data/{file_name}.txt", file_name = file_name);
        let input = fs::read_to_string(&file_path)?;

        println!("Mode: {:?}", mode);
        let part_1_score = solve_part_1(&input);
        println!("Part 1 - Solution: {score}", score = part_1_score);
        let part_2_score = solve_part_2(&input);
        println!("Part 2 - Solution: {score}", score = part_2_score);
    }

    Ok(())
}
