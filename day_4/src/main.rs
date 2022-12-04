use std::io::BufRead;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use file_utils::read_file;

lazy_static! {
    static ref LINE_PATTERN: Regex = Regex::new(r"^(\d+)-(\d+),[[:blank:]]*(\d+)-(\d+)$").unwrap();
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
            let first_assignment = Assignment::from(first_start, first_end).unwrap();

            let (seconds_start, second_end) = extract_assignment(&captures, 3, 4);
            let second_assignment = Assignment::from(seconds_start, second_end).unwrap();

            if first_assignment.contained_in(&second_assignment) {
                println!("Line {}: First assigment included: {:?}", line_index + 1, line_str)
            }
            if second_assignment.contained_in(&first_assignment) {
                println!("Line {}: Second assigment included: {:?}", line_index + 1, line_str)
            }
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
    start: u32,
    end: u32,
}

impl Assignment {
    fn from(start: u32, end: u32) -> Result<Assignment, String> {
        if start == 0 {
            return Err(String::from("start cannot be 0"));
        }
        if end == 0 {
            return Err(String::from("end cannot be 0"));
        }
        if start < end {
            Ok(Assignment { start, end })
        } else {
            Ok(Assignment { start: end, end: start })
        }
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
        let first = Assignment::from(1, 2).unwrap();
        let second = Assignment::from(3, 4).unwrap();
        let actual = first.contained_in(&second);
        assert_eq!(actual, false);
    }

    #[test]
    fn assigment_is_contained_and_equal() {
        let first = Assignment::from(2, 3).unwrap();
        let second = Assignment::from(2, 3).unwrap();
        let actual = first.contained_in(&second);
        assert_eq!(actual, true);
    }

    #[test]
    fn assigment_is_contained() {
        let first = Assignment::from(2, 3).unwrap();
        let second = Assignment::from(1, 4).unwrap();
        let actual = first.contained_in(&second);
        assert_eq!(actual, true);
    }

    #[test]
    fn assigment_is_not_contained_by_start() {
        let first = Assignment::from(1, 3).unwrap();
        let second = Assignment::from(2, 4).unwrap();
        let actual = first.contained_in(&second);
        assert_eq!(actual, false);
    }

    #[test]
    fn assigment_is_not_contained_by_end() {
        let first = Assignment::from(2, 5).unwrap();
        let second = Assignment::from(1, 4).unwrap();
        let actual = first.contained_in(&second);
        assert_eq!(actual, false);
    }

    #[test]
    fn auto_sorts_assigment() {
        let assigment = Assignment::from(1, 2).unwrap();
        assert_eq!(assigment.start, 1);
        assert_eq!(assigment.end, 2);

        let assigment = Assignment::from(2, 1).unwrap();
        assert_eq!(assigment.start, 1);
        assert_eq!(assigment.end, 2);
    }

    #[test]
    fn assigment_starts_cannot_be_0() {
        let error = Assignment::from(0, 1).unwrap_err();
        assert_eq!(error, "start cannot be 0");
    }

    #[test]
    fn assigment_end_cannot_be_0() {
        let error = Assignment::from(1, 0).unwrap_err();
        assert_eq!(error, "end cannot be 0");
    }
}
