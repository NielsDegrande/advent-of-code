use std::fs;

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No real solutions.
        None
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let x1 = (-b + sqrt_discriminant) / (2.0 * a);
        let x2 = (-b - sqrt_discriminant) / (2.0 * a);
        Some((x1, x2))
    }
}

fn is_integer(n: f64) -> bool {
    n.fract() == 0.0
}

fn find_race_options(time: &u64, distance: &u64) -> u64 {
    let (mut x1, mut x2) =
        solve_quadratic(-1.0, *time as f64, -(*distance as f64)).expect("No solution found.");
    if is_integer(x1) {
        x1 += 1.0;
    }
    if is_integer(x2) {
        x2 -= 1.0;
    }

    let min_time = x1.ceil() as u64;
    let max_time = x2.floor() as u64;
    max_time - min_time + 1
}

fn parse_data_part1(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut data = input.lines().map(|line| {
        line.split_once(":")
            .expect("No colon in line.")
            .1
            .split_whitespace()
            .map(|number| number.parse::<u64>().expect("Not a number."))
            .collect()
    });
    (
        data.next().expect("No times found."),
        data.next().expect("No distances found."),
    )
}

fn solve_part_1(input: &str) -> u64 {
    let (times, distances) = parse_data_part1(input);
    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| find_race_options(time, distance))
        .product()
}

fn parse_data_part2(input: &str) -> (u64, u64) {
    let mut data = input.lines().map(|line| {
        line.split_once(":")
            .expect("No colon in line.")
            .1
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<u64>()
            .expect("Not a number.")
    });
    (
        data.next().expect("No times found."),
        data.next().expect("No distances found."),
    )
}

fn solve_part_2(input: &str) -> u64 {
    let (time, distance) = parse_data_part2(input);
    find_race_options(&time, &distance)
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
