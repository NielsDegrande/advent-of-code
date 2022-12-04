use std::collections::HashMap;
use std::fs;
use std::path;

const PART: u8 = 2;

fn read_file(file_path_as_str: &str) -> String {
    let file_path = path::Path::new(file_path_as_str);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

fn create_score_mapping_part_1() -> HashMap<String, u32> {
    let mut scores = HashMap::new();

    // Brute force:
    //  A: Rock, B: Paper, C: Scissors.
    //  Part 1 - X: Rock -> 1, Y: Paper -> 2, Z: Scissors -> 3.
    scores.insert(String::from("A X"), 1 + 3);
    scores.insert(String::from("A Y"), 2 + 6);
    scores.insert(String::from("A Z"), 3 + 0);
    scores.insert(String::from("B X"), 1 + 0);
    scores.insert(String::from("B Y"), 2 + 3);
    scores.insert(String::from("B Z"), 3 + 6);
    scores.insert(String::from("C X"), 1 + 6);
    scores.insert(String::from("C Y"), 2 + 0);
    scores.insert(String::from("C Z"), 3 + 3);

    scores
}

fn create_score_mapping_part_2() -> HashMap<String, u32> {
    let mut scores = HashMap::new();

    // Brute force:
    //  Part 2 - X: Lose -> 0, Y: Draw -> 3, Z: Win -> 6.
    //  Rock: 1, Paper: 2, Scissors: 3.
    scores.insert(String::from("A X"), 3 + 0); // Rock -> Lose: Scissors.
    scores.insert(String::from("A Y"), 1 + 3); // Rock -> Draw: Rock.
    scores.insert(String::from("A Z"), 2 + 6); // Rock -> Win: Paper.
    scores.insert(String::from("B X"), 1 + 0); // Paper -> Lose: Rock.
    scores.insert(String::from("B Y"), 2 + 3); // Paper -> Draw: Paper.
    scores.insert(String::from("B Z"), 3 + 6); // Paper -> Win: Scissors.
    scores.insert(String::from("C X"), 2 + 0); // Scissors -> Lose: Paper.
    scores.insert(String::from("C Y"), 3 + 3); // Scissors -> Draw: Scissors.
    scores.insert(String::from("C Z"), 1 + 6); // Scissors -> Win: Rock.

    scores
}

fn main() {
    // Read file.
    let contents = read_file("data/input.txt");

    // Create score mapping.
    let scores: HashMap<String, u32>;
    if PART == 1 {
        scores = create_score_mapping_part_1();
    } else if PART == 2 {
        scores = create_score_mapping_part_2();
    } else {
        panic!("Invalid part!");
    }

    // Compute scores.
    let mut sum: u32 = 0;
    for line in contents.lines() {
        sum += scores[line];
    }
    println!("Total score: {}", sum);
}
