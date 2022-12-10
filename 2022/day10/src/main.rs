use std::fs;
use std::path::Path;

// Choose the mode: Test or Real.
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

fn increase_cycle(cycle: &mut i32, value_of_x: &i32) -> i32 {
    let signal_strength;
    *cycle += 1;
    if (*cycle - 20) % 40 == 0 {
        signal_strength = *cycle * value_of_x;
    } else {
        signal_strength = 0;
    }

    signal_strength
}

fn solve_part_1(contents: &String) -> i32 {
    // Keep track of solution.
    let mut cycle: i32 = 0;
    let mut value_of_x: i32 = 1;
    let mut signal_strength: i32 = 0;

    // Parse line by line.
    for line in contents.lines() {
        if line == "noop" {
            signal_strength += increase_cycle(&mut cycle, &value_of_x);
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            signal_strength += increase_cycle(&mut cycle, &value_of_x);
            signal_strength += increase_cycle(&mut cycle, &value_of_x);
            value_of_x += parts[1].parse::<i32>().unwrap();
        }
    }

    signal_strength
}

fn evaluate_screen(cycle: &i32, value_of_x: &i32) -> String {
    // Cycle is equivalent to pixel being drawn, but always one higher.
    let pixel_being_drawn = (cycle - 1) % 40;
    // Overlap between pixel and sprite means left, same or right pixel of x.
    if value_of_x == &(pixel_being_drawn - 1)
        || value_of_x == &pixel_being_drawn
        || value_of_x == &(pixel_being_drawn + 1)
    {
        "#".to_string()
    } else {
        ".".to_string()
    }
}

fn solve_part_2(contents: &String) {
    // Keep track of solution.
    let mut cycle: i32 = 1;
    let mut value_of_x: i32 = 1;
    let mut screen: Vec<String> = Vec::new();

    // Parse line by line.
    for line in contents.lines() {
        if line == "noop" {
            screen.push(evaluate_screen(&cycle, &value_of_x));
            cycle +=1;
        } else {
            screen.push(evaluate_screen(&cycle, &value_of_x));
            cycle +=1;
            screen.push(evaluate_screen(&cycle, &value_of_x));
            cycle +=1;
            let parts: Vec<&str> = line.split_whitespace().collect();
            value_of_x += parts[1].parse::<i32>().unwrap();
        }
    }

    // Print output.
    let mut line: String = "".to_owned();
    for i in 0..240 {
        if i % 40 == 0 {
            println!("{}", line);
            line = "".to_owned();
        }
        line.push_str(&screen[i]);
    }
    println!("{}", line);
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
    let part_1_score = solve_part_1(&contents);
    println!("Part 1 - Solution: {}", part_1_score);

    // Part 2.
    solve_part_2(&contents);
}
