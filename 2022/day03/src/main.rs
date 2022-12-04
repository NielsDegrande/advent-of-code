use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::Lines;

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

fn generate_alphabet_map() -> HashMap<char, usize> {
    // Generate alphabet.
    let mut alphabet: Vec<char> = ('a'..='z').into_iter().collect::<Vec<char>>();
    let upper_case_alphabet: Vec<char> = ('A'..='Z').into_iter().collect::<Vec<char>>();
    alphabet.extend(upper_case_alphabet);

    // Map to position.
    let mut alphabet_map: HashMap<char, usize> = HashMap::new();
    for (i, char) in alphabet.iter().enumerate() {
        alphabet_map.insert(*char, i + 1);
    }

    alphabet_map
}

fn compute_priorities_part_1(contents: &String, alphabet_map: &HashMap<char, usize>) -> usize {
    // Keep track of priorities.
    let mut priorities = 0;

    // Parse line by line.
    for line in contents.lines() {
        // Split lines by half.
        let half_length: usize = line.len() / 2;
        let first_half: &str = &line[..half_length];
        let second_half: &str = &line[half_length..];

        // De-duplicate first half to avoid double counting priorities.
        let mut unique_first_half: Vec<char> = first_half.chars().collect();
        unique_first_half.sort();
        unique_first_half.dedup();

        // Add priority of line to total priorities.
        for char in unique_first_half {
            if second_half.contains(char) {
                priorities += alphabet_map[&char];
            }
        }
    }

    priorities
}

fn compute_priorities_part_2(contents: &String, alphabet_map: &HashMap<char, usize>) -> usize {
    // Keep track of priorities.
    let mut priorities = 0;

    // Parse three lines at a time.
    let lines: Lines = contents.lines();
    for (line1, line2, line3) in lines.tuples() {
        // De-duplicate first string to avoid double counting priorities.
        let mut first_string: Vec<char> = line1.chars().collect();
        first_string.sort();
        first_string.dedup();

        // Add priority of line to total priorities.
        for char in first_string {
            if line2.contains(char) {
                if line3.contains(char) {
                    priorities += alphabet_map[&char];
                }
            }
        }
    }

    priorities
}

fn main() {
    // Read file.
    // let contents: String = read_file("data/test.txt");
    let contents: String = read_file("data/input.txt");

    // Generate alphabet map.
    let alphabet_map = generate_alphabet_map();

    // Part 1.
    let part_1_priorities = compute_priorities_part_1(&contents, &alphabet_map);
    println!("Part 1 - Priorities: {}", part_1_priorities);

    // Part 2.
    let part_2_priorities = compute_priorities_part_2(&contents, &alphabet_map);
    println!("Part 2 - Priorities: {}", part_2_priorities);
}
