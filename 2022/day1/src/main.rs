use std::fs;
use std::path;

const MAX_ELVES: usize = 3;

fn main() {
    let file_path = path::Path::new("data/input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut max: [i32; MAX_ELVES] = [0; MAX_ELVES];
    let mut sum = 0;

    for line in contents.lines() {
        if line == "" {
            if sum > max[0] {
                max[0] = sum;
                max.sort()
            }
            sum = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            sum += calories;
        }
    }
    let result: i32 = max.iter().sum();
    println!("Max: {}", result);
}
