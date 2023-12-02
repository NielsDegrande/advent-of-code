// Hide dead code and unused variables warning while developing.
// TODO: Remove before committing.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

fn solve_part_1(input: &str) -> u32 {
    input.lines().map(|_| 1).sum()
}

fn solve_part_2(input: &str) -> u32 {
    input.lines().map(|_| 1).sum()
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
