use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

// Choose the mode: Test or Real.
const CHOSEN_MODE: MODE = MODE::Real;
#[allow(dead_code)]
enum MODE {
    Test,
    Real,
}

// Magic numbers.
const CUTOFF_SIZE: u32 = 100000; // Size used as cutoff for part 1.
const DISK_SIZE: u32 = 70000000; // Total disk space available.
const UPDATE_SIZE: u32 = 30000000; // Free space required for update.

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

fn solve_puzzle(contents: &String) {
    let mut files: HashMap<PathBuf, u32> = HashMap::new();
    let mut current_path: PathBuf = PathBuf::new();
    // Parse line by line.
    // Create a map containing all absolute file paths and respective sizes.
    for line in contents.lines() {
        if line.contains("$ cd /") {
            current_path.push("/")
        } else if line.contains("$ cd ..") {
            current_path.pop();
        } else if line.contains("$ cd") {
            current_path = current_path.join(line.replace("$ cd ", ""));
        } else if line.contains("$ ls") || line.contains("dir") {
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let file_size: u32 = parts[0].parse().unwrap();
            let file_name: &str = parts[1];
            let mut file_path = current_path.clone();
            file_path.push(file_name);
            files.insert(file_path, file_size);
        }
    }

    // Loop over all file paths to compute directory sizes.
    let mut directories: HashMap<PathBuf, u32> = HashMap::new();
    let mut root_path = PathBuf::new();
    root_path.push("/");
    for (file_path, file_size) in files.iter() {
        let mut parent_path = file_path.clone();
        loop {
            parent_path.pop();
            let directory_size = match directories.get(&parent_path) {
                None => &0,
                Some(directory_size) => directory_size,
            };
            directories.insert(parent_path.clone(), directory_size + file_size);
            if parent_path == root_path {
                break;
            }
        }
    }

    // Part 1 - Compute sum of directories under cutoff.
    let mut size_counter: u32 = 0;
    for directory_size in directories.values() {
        if directory_size <= &CUTOFF_SIZE {
            size_counter += directory_size
        }
    }
    println!("Part 1 - Solution: {}", size_counter);

    // Part 2 - Find best (minimal delete that allows for update) directory.
    let total_used_space: &u32 = directories.get(&root_path).unwrap();
    let additional_space_required_for_update: u32 = UPDATE_SIZE - (DISK_SIZE - total_used_space);
    // Initialize to a high number, e.g., total disk size.
    let mut current_smallest_size = &DISK_SIZE;
    //
    for directory_size in directories.values() {
        if directory_size >= &additional_space_required_for_update
            && directory_size < &current_smallest_size
        {
            current_smallest_size = directory_size;
        }
    }
    println!("Part 2 - Solution: {}", current_smallest_size);
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

    // Solve part 1 and 2.
    solve_puzzle(&contents);
}
