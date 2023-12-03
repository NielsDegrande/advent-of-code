use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

const RADIX: u32 = 10;

fn to_2d_vector(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn find_adjacent_symbol(symbols: &Vec<Vec<bool>>, row_index: usize, column_index: usize) -> bool {
    let row_min = row_index.saturating_sub(1);
    let row_max = std::cmp::min(row_index + 1, symbols.len() - 1);
    let col_min = column_index.saturating_sub(1);
    let col_max = std::cmp::min(column_index + 1, symbols[0].len() - 1);

    for r in row_min..=row_max {
        for c in col_min..=col_max {
            // Avoid checking the cell itself.
            if r == row_index && c == column_index {
                continue;
            }
            if symbols[r][c] {
                return true;
            }
        }
    }

    false
}

fn solve_part_1(input: &str) -> u32 {
    let board: Vec<Vec<char>> = to_2d_vector(input);
    let symbols: Vec<Vec<bool>> = board
        .iter()
        .map(|row| {
            row.iter()
                .map(|&c| !(c.is_digit(RADIX) || c == '.'))
                .collect()
        })
        .collect();

    let re = Regex::new(r"\d+").unwrap();
    let mut numbers_to_accumulate = Vec::new();
    for (row_index, line) in input.lines().enumerate() {
        for capture in re.find_iter(line) {
            let start_column_index = capture.start();
            for (character_index, _) in capture.as_str().chars().enumerate() {
                let column_index = start_column_index + character_index;
                if find_adjacent_symbol(&symbols, row_index, column_index) {
                    numbers_to_accumulate.push(capture.as_str().parse::<u32>().unwrap());
                    break;
                }
            }
        }
    }
    numbers_to_accumulate.iter().sum()
}

fn find_adjacent_gear(
    matrix: &Vec<Vec<bool>>,
    row_index: usize,
    start_column_index: usize,
    capture: &str,
    parts: &mut HashMap<(usize, usize), Vec<u32>>,
) {
    let row_min = row_index.saturating_sub(1);
    let row_max = std::cmp::min(row_index + 1, matrix.len() - 1);

    for (character_index, _) in capture.chars().enumerate() {
        let column_index = start_column_index + character_index;
        let col_min = column_index.saturating_sub(1);
        let col_max = std::cmp::min(column_index + 1, matrix[0].len() - 1);
        for r in row_min..=row_max {
            for c in col_min..=col_max {
                // Avoid checking the cell itself.
                if r == row_index && c == column_index {
                    continue;
                }
                if matrix[r][c] {
                    parts
                        .entry((r, c))
                        .or_insert_with(Vec::new)
                        .push(capture.parse::<u32>().unwrap());
                    return;
                }
            }
        }
    }
}

fn solve_part_2(input: &str) -> u32 {
    let board: Vec<Vec<char>> = to_2d_vector(input);
    let symbols: Vec<Vec<bool>> = board
        .iter()
        .map(|row| row.iter().map(|&c| c == '*').collect())
        .collect();

    let re = Regex::new(r"\d+").unwrap();
    let mut parts: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (row_index, line) in input.lines().enumerate() {
        for capture in re.find_iter(line) {
            let start_column_index = capture.start();
            find_adjacent_gear(
                &symbols,
                row_index,
                start_column_index,
                capture.as_str(),
                &mut parts,
            );
        }
    }

    parts
        .values()
        .filter(|part| part.len() == 2)
        .map(|part| part[0] * part[1])
        .sum()
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
