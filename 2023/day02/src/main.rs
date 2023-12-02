use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

fn parse_line(line: &str) -> Vec<HashMap<&str, u32>> {
    line.split_once(":")
        .unwrap()
        .1
        .split(';')
        .map(|part| {
            part.split(',')
                .map(|color_number_pair| {
                    let color_number_pair = color_number_pair.trim();
                    let (number, color) = color_number_pair.split_once(' ').unwrap();
                    (color.trim(), number.trim().parse::<u32>().unwrap())
                })
                .collect::<HashMap<&str, u32>>()
        })
        .collect()
}

fn solve_part_1(input: &str) -> u32 {
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            parse_line(line)
                .iter()
                .all(|draw| draw.iter().all(|(color, number)| bag[color] >= *number))
                .then(|| (index + 1) as u32)
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut max_by_ball = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            parse_line(line)
                .iter()
                .flatten()
                .for_each(|(color, &number)| {
                    max_by_ball
                        .entry(color)
                        .and_modify(|max_value: &mut u32| *max_value = (*max_value).max(number));
                });
            max_by_ball.values().product::<u32>()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modes = [Mode::TEST, Mode::REAL];

    for mode in modes.iter() {
        let file_name = match mode {
            Mode::TEST => "test",
            Mode::REAL => "input",
        };
        let file_path = format!("data/{}.txt", file_name);
        let input = fs::read_to_string(&file_path)?;

        println!("Mode: {:?}", mode);
        let part_1_score = solve_part_1(&input);
        println!("Part 1 - Solution: {score}", score = part_1_score);
        let part_2_score = solve_part_2(&input);
        println!("Part 2 - Solution: {score}", score = part_2_score);
    }

    Ok(())
}
