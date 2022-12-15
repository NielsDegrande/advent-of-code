use std::cmp;
use std::collections::HashSet;

// Choose the mode: Test or Real.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

// Magic numbers.
const TUNING_MULTIPLIER: i64 = 4_000_000;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn split_line(line: &str) -> (Point, Point) {
    // Remove unnecessary information.
    let parsed_line = line
        .replace("Sensor at x=", "")
        .replace(", y", "")
        .replace(": closest beacon is at x", "");

    // Split on structure: sensor.x, sensor.y, beacon.x, beacon.y.
    let parsed_splits: Vec<&str> = parsed_line.split("=").collect();
    let mut coordinates: Vec<i64> = Vec::new();
    for split in parsed_splits {
        coordinates.push(split.parse().unwrap());
    }

    // Load splits into points.
    let sensor: Point = Point {
        x: coordinates[0],
        y: coordinates[1],
    };
    let beacon: Point = Point {
        x: coordinates[2],
        y: coordinates[3],
    };

    (sensor, beacon)
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    let mut parsed_input: Vec<(Point, Point)> = Vec::new();
    for line in input.lines() {
        parsed_input.push(split_line(line));
    }

    parsed_input
}

fn compute_manhattan_distance(point_1: &Point, point_2: &Point) -> i64 {
    (point_1.x - point_2.x).abs() + (point_1.y - point_2.y).abs()
}

fn solve_part_1(data: &Vec<(Point, Point)>, solution_row: i64) -> u64 {
    // Initialize data structures.
    let mut coverage: HashSet<i64> = HashSet::new();
    let mut beacons: HashSet<i64> = HashSet::new();

    // Loop over all sensors and beacons.
    for (sensor, beacon) in data {
        // Keep track of beacons on solution_row as these are possible beacon locations.
        if beacon.y == solution_row {
            beacons.insert(beacon.x);
        }
        // Compute distance between beacon and sensor.
        let distance = compute_manhattan_distance(sensor, beacon);
        if sensor.y <= solution_row && solution_row <= sensor.y + distance
            || sensor.y - distance <= solution_row && solution_row <= sensor.y
        {
            let most_left_coverage = sensor.x - distance + (solution_row - sensor.y).abs();
            let most_right_coverage = sensor.x + distance - (solution_row - sensor.y).abs();
            // Find all points on solution_row that have coverage and add them to the coverage set.
            coverage.extend(most_left_coverage..most_right_coverage + 1);
        }
    }

    // Find out in how many spots the beacon cannot be (all covered locations without a beacon).
    return (coverage.len() - beacons.len()) as u64;
}

fn solve_part_2(data: &Vec<(Point, Point)>, solution_space_size: i64) -> u64 {
    let mut sensor_range: Vec<(i64, i64, i64)> = Vec::new();

    // Loop over all sensors and beacons.
    for (sensor, beacon) in data {
        // Compute distance between beacon and sensor.
        let distance = compute_manhattan_distance(sensor, beacon);
        sensor_range.push((sensor.x, sensor.y, distance));
    }

    // For all sensors, go one point outside the equal-distance box around the sensor.
    // If that point does not lie in an area covered by any other sensor,
    // then it has to be the distress beacon.
    for (sensor_x, sensor_y, distance) in &sensor_range {
        // Generate all possible horizontal positions (going one point beyond).
        // Limit points to the solution space.
        let left_x = cmp::max(0, sensor_x - distance - 1);
        let right_x = cmp::min(solution_space_size, sensor_x + distance + 1);
        for x in left_x..right_x + 1 {
            // Find corresponding vertical extremities (going one point beyond).
            let unused_distance = distance - (x - sensor_x).abs().abs();
            let down_y = sensor_y + unused_distance + 1;
            let up_y = sensor_y - unused_distance - 1;

            // Validate if either of the extremities lies outside all covered areas.
            for &y in [down_y, up_y].iter() {
                // Only consider extremities within the solution space.
                if y >= 0 && y <= solution_space_size {
                    let mut no_overlap: bool = true;

                    // Loop over all other sensors to compare extremity with all covered areas.
                    for (other_sensor_x, other_sensor_y, other_distance) in &sensor_range {
                        // Compute manhattan distance between the other sensor and the extremity.
                        // If it is within that sensor's range, then stop exploring this extremity.
                        if (other_sensor_x - x).abs() + (other_sensor_y - y).abs()
                            <= *other_distance
                        {
                            no_overlap = false;
                            break;
                        }
                    }

                    // If the extremity does not overlap with any covered area, return the tuning frequency.
                    if no_overlap {
                        return (x * TUNING_MULTIPLIER + y) as u64;
                    }
                }
            }
        }
    }

    // Return 0 if not solution can be found.
    0
}

fn main() {
    // Initialize problem.
    let input: &str;
    let solution_row: i64;
    let solution_space_size: i64;
    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            input = include_str!("../data/test.txt");
            solution_row = 10;
            solution_space_size = 20;
        }
        MODE::Real => {
            // Read input file.
            input = include_str!("../data/input.txt");
            solution_row = 2_000_000;
            solution_space_size = 4_000_000;
        }
    }

    // Parse input.
    let data: Vec<(Point, Point)> = parse_input(input);

    // Part 1.
    let part_1_score = solve_part_1(&data, solution_row);
    println!("Part 1 - Solution: {}", part_1_score);

    // Part 2.
    let part_2_score = solve_part_2(&data, solution_space_size);
    println!("Part 2 - Solution: {}", part_2_score);
}
