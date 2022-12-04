use std::io::BufRead;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use file_utils::read_file;

lazy_static! {
    // TODO support blank
    static ref LINE_PATTERN: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}

fn main() {
    find_overlaps();
}

fn is_valid_line(line: &String) -> bool {
    LINE_PATTERN.is_match(line)
}

fn find_overlaps() {
    let reader = read_file();

    for (line_index, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();

        if is_valid_line(&line_str) {
            // no need for error handling, we know line matches
            let captures = LINE_PATTERN.captures(&line_str).unwrap();

            let (first_start, first_end) = extract_assignment(&captures, 1, 2);
            let first_assignment = Assignment::from(first_start, first_end);
            let (seconds_start, second_end) = extract_assignment(&captures, 3, 4);
            let second_assignment = Assignment::from(seconds_start, second_end);

            if first_assignment.contained_in(&second_assignment) {
                println!("Line {}: First assigment included: {:?}", line_index + 1, line_str)
            }
            if second_assignment.contained_in(&first_assignment) {
                println!("Line {}: Second assigment included: {:?}", line_index + 1, line_str)
            }
            // Count from 0 as normal people do
        }
    }
}

fn extract_assignment(captures: &Captures, capture1: usize, capture2: usize) -> (u32, u32) {
    let value1: u32 = captures.get(capture1).unwrap().as_str().parse().unwrap();
    let value2: u32 = captures.get(capture2).unwrap().as_str().parse().unwrap();
    (value1, value2)
}

#[derive(Debug)]
struct Assignment {
    // TODO start >= 1
    start: u32,
    end: u32,
}

impl Assignment {
    fn from(start: u32, end: u32) -> Assignment {
        Assignment { start, end }
    }

    fn contained_in(&self, other: &Assignment) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

mod file_utils {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    pub(crate) fn read_file() -> BufReader<File> {
        BufReader::new(get_file())
    }

    fn get_file() -> File {
        let home_dir = env::var("HOME").unwrap();
        let path = "github/adventofcode-2022/day_4/src";
        let filename = "section_assignments.log";
        let full_path = format!("{}/{}/{}", home_dir, path, filename);
        File::open(full_path).unwrap()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod assigment_tests {
    use crate::Assignment;

    #[test]
    fn assigment_not_contained() {
        let first = Assignment::from(1, 2);
        let second = Assignment::from(3, 4);
        let actual = first.contained_in(&second);
        assert_eq!(actual, false);
    }

    #[test]
    fn assigment_is_contained_and_equal() {
        let first = Assignment::from(2, 3);
        let second = Assignment::from(2, 3);
        let actual = first.contained_in(&second);
        assert_eq!(actual, true);
    }

    #[test]
    fn assigment_is_contained() {
        let first = Assignment::from(2, 3);
        let second = Assignment::from(1, 4);
        let actual = first.contained_in(&second);
        assert_eq!(actual, true);
    }

    #[test]
    fn assigment_is_not_contained_by_start() {
        let first = Assignment::from(0, 3);
        let second = Assignment::from(1, 4);
        let actual = first.contained_in(&second);
        assert_eq!(actual, false);
    }

    #[test]
    fn assigment_is_not_contained_by_end() {
        let first = Assignment::from(2, 5);
        let second = Assignment::from(1, 4);
        let actual = first.contained_in(&second);
        assert_eq!(actual, false);
    }
}
