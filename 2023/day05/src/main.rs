use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

const DATA_CATEGORIES: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn read_seeds(file_name_prefix: &str) -> Vec<u64> {
    let file_path = format!(
        "data/{file_name_prefix}_seeds.txt",
        file_name_prefix = file_name_prefix,
    );
    let input = fs::read_to_string(&file_path).expect("File not found.");
    input
        .lines()
        .next()
        .expect("No lines in file.")
        .split_whitespace()
        .map(|number| number.parse::<u64>().expect("Failed to parse number."))
        .collect()
}

fn read_categories(file_name_prefix: &str) -> HashMap<&str, Vec<(u64, u64, u64)>> {
    let mut data_map: HashMap<&str, Vec<(u64, u64, u64)>> = HashMap::new();
    DATA_CATEGORIES.iter().for_each(|category| {
        let file_path = format!(
            "data/{file_name_prefix}_{category}.txt",
            file_name_prefix = file_name_prefix,
            category = category
        );
        let input = fs::read_to_string(&file_path).unwrap();
        let data: Vec<(u64, u64, u64)> = input
            .lines()
            .map(|line| {
                let line_data = line
                    .split_whitespace()
                    .map(|number| number.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                (line_data[0], line_data[1], line_data[2])
            })
            .collect();

        data_map.insert(&category, data);
    });
    data_map
}

fn find_location(seed: &u64, data_map: &HashMap<&str, Vec<(u64, u64, u64)>>) -> u64 {
    DATA_CATEGORIES
        .iter()
        .fold(*seed, |current_key, &category| {
            let category_data = data_map
                .get(category)
                .expect("Category not found in data_map.");
            category_data
                .iter()
                .find_map(|&(destination_start, source_start, source_range)| {
                    if source_start <= current_key && current_key < source_start + source_range {
                        Some(destination_start + current_key - source_start)
                    } else {
                        None
                    }
                })
                .unwrap_or(current_key)
        })
}

fn solve_part_1(file_name_prefix: &str) -> u64 {
    let seeds = read_seeds(file_name_prefix);
    let data_map = read_categories(file_name_prefix);
    seeds
        .iter()
        .map(|seed| find_location(seed, &data_map))
        .min()
        .expect("No minimum exists.")
}

fn solve_part_2(file_name_prefix: &str) -> u64 {
    let seeds: Vec<(u64, u64)> = read_seeds(file_name_prefix)
        .chunks(2)
        .map(|chunk: &[u64]| (chunk[0], chunk[1]))
        .collect();
    let data_map = read_categories(file_name_prefix);

    seeds
        .iter()
        .flat_map(|&(seed_start, seed_range)| {
            (seed_start..seed_start + seed_range).map(|seed| find_location(&seed, &data_map))
        })
        .min()
        .expect("No minimum exists.")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modes = [Mode::TEST, Mode::REAL];

    for mode in modes.iter() {
        let file_name_prefix = match mode {
            Mode::TEST => "test",
            Mode::REAL => "input",
        };

        println!("Mode: {:?}", mode);
        let part_1_score = solve_part_1(file_name_prefix);
        println!("Part 1 - Solution: {score}", score = part_1_score);
        let part_2_score = solve_part_2(&file_name_prefix);
        println!("Part 2 - Solution: {score}", score = part_2_score);
    }

    Ok(())
}
