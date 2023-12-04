use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let parts: Vec<Vec<u32>> = line
        .split_once(":")
        .unwrap()
        .1
        .split("|")
        .map(|list| {
            list.trim()
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    (
        parts.get(0).cloned().unwrap(),
        parts.get(1).cloned().unwrap(),
    )
}

fn count_winning_numbers(winning_numbers: &Vec<u32>, numbers: &Vec<u32>) -> usize {
    winning_numbers
        .iter()
        .filter(|&number| numbers.contains(number))
        .count()
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line(line))
        .map(|(part1, part2)| {
            let count = count_winning_numbers(&part1, &part2);
            if count > 0 {
                2u32.pow(count as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    let mut cards = HashMap::new();
    input.lines().enumerate().for_each(|(line_index, line)| {
        let copies = cards.entry(line_index).or_insert(1);
        let (part1, part2) = parse_line(line);
        let count = count_winning_numbers(&part1, &part2);
        let new_copies = copies.clone();
        (1..=count).for_each(|copy_index| {
            *cards.entry(line_index + copy_index).or_insert(1) += new_copies;
        });
    });
    cards.values().sum()
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
