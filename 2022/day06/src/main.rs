use std::collections::HashSet;
use std::fs;
use std::path::Path;

// Choose the mode: Real or Test.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

fn solve_puzzle(contents: &String, number_of_unique_characters: usize) {
    // Parse line by line.
    for line in contents.lines() {
        // Keep track of last n characters.
        let mut last_n_characters: Vec<char> = vec!['a'; number_of_unique_characters];
        // Loop over characters in input line.
        for (index, char) in line.chars().enumerate() {
            // First populate the vector.
            if index < number_of_unique_characters {
                last_n_characters[index] = char;
            } else {
                // Create a set to find the unique characters in the vector.
                let last_unique_n_characters: HashSet<char> =
                    HashSet::from_iter(last_n_characters.iter().cloned());
                // If all last n are unique, then print the solution.
                if last_n_characters.len() == last_unique_n_characters.len() {
                    println!("Solution (with {}): {}", number_of_unique_characters, index);
                    break;
                // Else replace the relevant item in the last n character vector (cycling around).
                } else {
                    let index_to_update = index %  number_of_unique_characters;
                    last_n_characters[index_to_update] = char;
                }
            }
        }
    }
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

    // Part 1.
    solve_puzzle(&contents, 4);

    // Part 2.
    solve_puzzle(&contents, 14);
}
