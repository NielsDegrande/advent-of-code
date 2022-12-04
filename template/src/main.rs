use std::fs;
use std::path::Path;

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

fn solve_part_1(contents: &String) -> usize {
    // Keep track of score.
    let mut score = 0;

    // Parse line by line.
    for line in contents.lines() {
        println!("{}", line);
        score += 1
    }
    score
}

fn solve_part_2(contents: &String) -> usize {
    // Keep track of score.
    let mut score = 0;

    // Parse line by line.
    for line in contents.lines() {
        println!("{}", line);
        score += 1
    }
    score
}

fn main() {
    // Read file.
    let contents: String = read_file("data/test.txt");
    // let contents: String = read_file("data/input.txt");

    // Part 1.
    let part_1_score = solve_part_1(&contents);
    println!("Part 1 - Score: {}", part_1_score);

    // Part 2.
    let part_2_score = solve_part_2(&contents);
    println!("Part 2 - Score: {}", part_2_score);
}
