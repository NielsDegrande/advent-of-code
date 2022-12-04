use std::fs;
use std::path::Path;

fn read_file(file_path_as_str: &str) -> String {
    let file_path: &Path = Path::new(file_path_as_str);
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file.");
    contents
}

struct Section {
    start: u32,
    end: u32,
}

fn parse_line(line: &str) -> (Section, Section) {
    // Split the line in pair sections.
    let pairs: Vec<&str> = line.split(",").collect();
    // Parse the start and end of both sections.
    let first_pair: Vec<&str> = pairs[0].split("-").collect();
    let second_pair: Vec<&str> = pairs[1].split("-").collect();

    // Map first section data in Section struct.
    let first_section = Section {
        start: first_pair[0].parse().unwrap(),
        end: first_pair[1].parse().unwrap(),
    };
    // Map second section data in Section struct.
    let second_section = Section {
        start: second_pair[0].parse().unwrap(),
        end: second_pair[1].parse().unwrap(),
    };

    (first_section, second_section)
}

fn find_overlaps_part_1(contents: &String) -> usize {
    // Keep track of overlaps.
    let mut overlaps = 0;

    // Parse line by line.
    for line in contents.lines() {
        let (first_section, second_section) = parse_line(line);

        if first_section.start <= second_section.start && first_section.end >= second_section.end {
            overlaps += 1
        } else if second_section.start <= first_section.start
            && second_section.end >= first_section.end
        {
            overlaps += 1
        }
    }
    overlaps
}

fn find_overlaps_part_2(contents: &String) -> usize {
    // Keep track of overlaps.
    let mut overlaps = 0;

    // Parse line by line.
    for line in contents.lines() {
        let (first_section, second_section) = parse_line(line);

        // First start encompassed in Second: second_start <= first_start <= second_end.
        if second_section.start <= first_section.start && first_section.start <= second_section.end
        {
            overlaps += 1
        }
        // First end encompassed in Second: second_start <= first_end <= second_end
        else if second_section.start <= first_section.end
            && first_section.end <= second_section.end
        {
            overlaps += 1
        }
        // Second start encompassed in First: first_start <= second_start <= first_end.
        else if first_section.start <= second_section.start
            && second_section.start <= first_section.end
        {
            overlaps += 1
        }
        // Second end encompassed in First: first_start <= second_end <= first_end.
        else if first_section.start <= second_section.end
            && second_section.end <= first_section.end
        {
            overlaps += 1
        }
    }
    overlaps
}

fn main() {
    // Read file.
    // let contents: String = read_file("data/test.txt");
    let contents: String = read_file("data/input.txt");

    // Part 1.
    let part_1_overlaps = find_overlaps_part_1(&contents);
    println!("Part 1 - Overlaps: {}", part_1_overlaps);

    // Part 2.
    let part_2_overlaps = find_overlaps_part_2(&contents);
    println!("Part 2 - Overlaps: {}", part_2_overlaps);
}
