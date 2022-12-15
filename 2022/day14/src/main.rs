use std::cmp;

// Choose the mode: Test or Real.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn split_line(line: &str) -> Vec<Point> {
    let input_points: Vec<&str> = line.split(" -> ").collect();
    let mut points: Vec<Point> = Vec::new();

    for input_point in input_points {
        let parts: Vec<&str> = input_point.split(",").collect();
        let point: Point = Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        };
        points.push(point);
    }

    points
}

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    let mut parsed_input: Vec<Vec<Point>> = Vec::new();
    for line in input.lines() {
        parsed_input.push(split_line(line));
    }

    parsed_input
}

fn find_grid_dimensions(data: &Vec<Vec<Point>>, start_point: &Point) -> (i32, i32, i32, i32) {
    let mut min_x: i32 = i32::MAX;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = i32::MAX;
    let mut max_y: i32 = 0;

    for path in data {
        for point in path {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }
    }

    // Ensure bounds.
    min_x = cmp::min(min_x, start_point.x);
    max_x = cmp::max(max_x, start_point.x);
    min_y = cmp::min(min_y, start_point.y);
    max_y = cmp::max(max_y, start_point.y);

    (min_x, max_x, min_y, max_y)
}

fn normalize_point(point: &Point, min_x: &i32, min_y: &i32) -> (usize, usize) {
    let normalized_x = (point.x - min_x) as usize;
    let normalized_y = (point.y - min_y) as usize;

    (normalized_x, normalized_y)
}

fn create_grid(
    start_point: &Point,
    min_x: &i32,
    max_x: &i32,
    min_y: &i32,
    max_y: &i32,
    data: &Vec<Vec<Point>>,
) -> Vec<Vec<char>> {
    // Create grid filled with air (`.`).
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    for _ in *min_x..(max_x + 1) {
        row.push('.');
    }
    for _ in *min_y..(max_y + 1) {
        grid.push(row.clone());
    }

    // Add source.
    let (normalized_source_x, normalized_source_y) = normalize_point(start_point, min_x, min_y);
    grid[normalized_source_y][normalized_source_x] = '+';

    // Add rocks to the grid.
    for path in data {
        for (i, point) in path.iter().enumerate() {
            if i == 0 {
                let (normalized_x, normalized_y) = normalize_point(point, min_x, min_y);
                grid[normalized_y][normalized_x] = '#';
            } else {
                let previous_point = path[i - 1];
                let x_distance = point.x - previous_point.x;
                let y_distance = point.y - previous_point.y;
                // Move in y direction.
                if x_distance == 0 {
                    if y_distance > 0 {
                        // Move down.
                        for y in previous_point.y..point.y + 1 {
                            let next_point: Point = Point { x: point.x, y: y };
                            let (normalized_x, normalized_y) =
                                normalize_point(&next_point, min_x, min_y);
                            grid[normalized_y][normalized_x] = '#';
                        }
                    }
                    if y_distance < 0 {
                        // Move up.
                        for y in point.y..previous_point.y + 1 {
                            let next_point: Point = Point { x: point.x, y: y };
                            let (normalized_x, normalized_y) =
                                normalize_point(&next_point, min_x, min_y);
                            grid[normalized_y][normalized_x] = '#';
                        }
                    }
                }
                // Move in x direction.
                if y_distance == 0 {
                    if x_distance > 0 {
                        // Move right.
                        for x in previous_point.x..point.x + 1 {
                            let next_point: Point = Point { x: x, y: point.y };
                            let (normalized_x, normalized_y) =
                                normalize_point(&next_point, min_x, min_y);
                            grid[normalized_y][normalized_x] = '#';
                        }
                    }
                    if x_distance < 0 {
                        // Move left.
                        for x in point.x..previous_point.x + 1 {
                            let next_point: Point = Point { x: x, y: point.y };
                            let (normalized_x, normalized_y) =
                                normalize_point(&next_point, min_x, min_y);
                            grid[normalized_y][normalized_x] = '#';
                        }
                    }
                }
            }
        }
    }

    grid
}

fn visualize_grid(grid: &Vec<Vec<char>>, min_x: &i32) {
    let mut top_line_string: String = String::new();
    top_line_string.push_str("    ");
    for i in 0..grid[0].len() {
        top_line_string.push_str(&((i as i32 + min_x) % 10).to_string());
    }
    println!("{}", top_line_string);

    for (i, row) in grid.iter().enumerate() {
        let mut print_string: String = String::new();
        print_string.push_str(&format!("{:03} ", i));
        for character in row {
            print_string.push(*character);
        }
        println!("{}", print_string);
    }
}

fn solve(
    grid: &mut Vec<Vec<char>>,
    start_point: &Point,
    min_x: &i32,
    min_y: &i32,
    part: u8,
) -> u32 {
    // Keep track of the solution.
    let mut solution = 0;

    // Loop over sand particles until it flows out of the bottom.
    loop {
        // Create a new normalized sand particle.
        let (mut x, mut y) = normalize_point(&start_point, min_x, min_y);

        // Sand drops one unit at a time.
        loop {
            // Fall down if possible.
            if y + 1 < grid.len() && grid[y + 1][x] == '.' {
                y += 1;
            }
            // Else, fall down-left if possible.
            else if y + 1 < grid.len() && x > 0 && grid[y + 1][x - 1] == '.' {
                y += 1;
                x -= 1;
            }
            // Else, fall down-right.
            else if y + 1 < grid.len() && x + 1 < grid[0].len() && grid[y + 1][x + 1] == '.' {
                y += 1;
                x += 1;
            }
            // Stop if a sand particle would fall outside of the grid.
            else if part == 1 && (x == 0 || x == grid[0].len() - 1 || y == grid.len() - 1) {
                visualize_grid(&grid, &min_x);
                return solution;
            } else if part == 2 && grid[y][x] == '+' {
                // Add one to block the source entirely.
                return solution + 1;
            }
            // Else, rest.
            else if grid[y][x] == '.' {
                grid[y][x] = 'o';
                break;
            } else {
                println!("WARNING: This should not happen!");
            }
        }
        solution += 1;
    }
}

fn main() {
    // Initialize problem.
    let input: &str;
    match CHOSEN_MODE {
        MODE::Test => {
            // Read test file.
            input = include_str!("../data/test.txt");
        }
        MODE::Real => {
            // Read input file.
            input = include_str!("../data/input.txt");
        }
    }

    let data: Vec<Vec<Point>> = parse_input(input);

    // Initialize and visualize grid.
    let start_point = Point { x: 500, y: 0 };
    let (min_x, max_x, min_y, max_y) = find_grid_dimensions(&data, &start_point);
    let grid: Vec<Vec<char>> = create_grid(&start_point, &min_x, &max_x, &min_y, &max_y, &data);

    // Part 1.
    let part_1_score = solve(&mut grid.clone(), &start_point, &min_x, &min_y, 1);
    println!("Part 1 - Solution: {}", part_1_score);

    // Part 2.
    // Mimic infinity by adding a very large number (large compared to size of max_x).
    let enlarged_min_x = min_x - (max_x * 4);
    let enlarged_max_x = max_x + (max_x * 4);
    // Increase floor with + 2 beyond highest point seen so far..
    let lowered_floor = max_y + 2;

    let mut grid: Vec<Vec<char>> = create_grid(
        &start_point,
        &enlarged_min_x,
        &enlarged_max_x,
        &min_y,
        &lowered_floor,
        &data,
    );

    // Fill the floor with rocks.
    let bottom_row = grid.len() - 1;
    let row_length = grid[0].len();
    for i in 0..row_length {
        grid[bottom_row][i] = '#';
    }

    let part_2_score = solve(&mut grid.clone(), &start_point, &enlarged_min_x, &min_y, 2);
    println!("Part 2 - Solution: {}", part_2_score);
}
