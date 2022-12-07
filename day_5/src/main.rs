use std::collections::VecDeque;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use file_utils::read_file;

lazy_static! {
    static ref MOVEMENT_PATTERN: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    static ref STACK_NUMBERS: Regex = Regex::new(r"^( *(\d+) *)*$").unwrap();
}

fn main() {
    operate_crane();
}

fn is_movement_line(line: &String) -> bool {
    MOVEMENT_PATTERN.is_match(line) || is_comment(line)
}

fn is_comment(line: &String) -> bool {
    line.starts_with("//")
}

fn operate_crane() {
    let reader = read_file();

    let mut movements: VecDeque<Movement> = VecDeque::new();
    let mut stacks: Vec<Stack> = Vec::new();

    let mut lines: Vec<String> = reader.lines()
        .map(|line| { line.unwrap() })
        .collect();

    let mut i = lines.len() - 1;
    while i >= 0 {
        let mut line_str = lines.get(i).unwrap();
        // process movements
        while is_movement_line(line_str) {
            // no need for error handling, we know line matches
            movements.push_front(Movement::parse(line_str));
            i -= 1;
            line_str = lines.get(i).unwrap();
        }
        // process separation
        while is_skippable(line_str) {
            i -= 1;
            line_str = lines.get(i).unwrap();
        };

        // process stack numbers
        println!("Processing {}", line_str);
        let captures = STACK_NUMBERS.captures(&line_str).unwrap();
        let x1 = captures.get(0).unwrap().as_str();
        let x2 = captures.get(1).unwrap().as_str();
        let x3 = captures.get(2).unwrap().as_str();
        let x4 = captures.get(3).unwrap().as_str();
        let x5 = captures.get(4).unwrap().as_str();

        println!("{:?}", captures);
        while is_crates_line(line_str) {}
    }
}

fn is_crates_line(line: &str) -> bool {
    true
}

fn is_skippable(line: &str) -> bool {
    line.is_empty()
}

// TODO ignore or fail when quantity is 0
struct Movement {
    quantity: u32,
    source_stack: u32,
    target_stack: u32,
}

impl Movement {
    fn parse(line: &String) -> Movement {
        let captures = MOVEMENT_PATTERN.captures(line).unwrap();

        Movement {
            quantity: captures.get(1).unwrap().as_str().parse().unwrap(),
            source_stack: captures.get(2).unwrap().as_str().parse().unwrap(),
            target_stack: captures.get(3).unwrap().as_str().parse().unwrap(),
        }
    }
}

struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Stack { crates: Vec::new() }
    }
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
        let path = "github/adventofcode-2022/day_5/src";
        let filename = "crane_moves.log";
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
