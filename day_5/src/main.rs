use std::collections::VecDeque;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use file_utils::read_file;

lazy_static! {
    static ref MOVEMENT_PATTERN: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    static ref STACK_NUMBERS: Regex = Regex::new(r"^\s*(\d+)\s+(\d+)(?:\s+(\d+))*\s*$").unwrap();
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
        .map(|line| line.unwrap())
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
        let vec = parse_stack_numbers(&line_str);

        if (vec.len() == 0) {
            println!("No valid stack numbers found");
            return;
        }

        while is_crates_line(line_str) {}
    }
}

fn parse_stack_numbers(line: &str) -> Vec<i32> {
    if STACK_NUMBERS.is_match(&line) {
        let captures = STACK_NUMBERS.captures(&line).unwrap();

        return captures.iter()
            .skip(1)
            .map(|c| {
                c.unwrap().as_str().parse().unwrap()
            })
            .collect();
    }
    Vec::new()
}

// TODO piles id in input are not in order

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

#[cfg(test)]
mod stack_numbers_parser {
    use crate::parse_stack_numbers;

    #[test]
    fn parses_empty_line() {
        let line = "";
        let vec = parse_stack_numbers(line);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn parses_invalid_line_only_text() {
        let line = "hello";
        let vec = parse_stack_numbers(line);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn parses_invalid_line_mixed_text() {
        let line = "1 2 3 a";
        let vec = parse_stack_numbers(line);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn parses_invalid_line_text_with_numbers() {
        let line = "1 2a 3";
        let vec = parse_stack_numbers(line);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn parses_simple_line() {
        let line = "1 2 3";
        let vec = parse_stack_numbers(line);
        assert_eq!(vec.len(), 3);
        assert_eq!(value_at(&vec, 0), 1);
        assert_eq!(value_at(&vec, 1), 2);
        assert_eq!(value_at(&vec, 2), 3);
    }

    #[test]
    fn parses_complex_line() {
        let line = "    11    232     323 ";
        let vec = parse_stack_numbers(line);
        assert_eq!(vec.len(), 3);
        assert_eq!(value_at(&vec, 0), 11);
        assert_eq!(value_at(&vec, 1), 232);
        assert_eq!(value_at(&vec, 2), 323);
    }

    fn value_at(vec: &Vec<i32>, index: usize) -> i32 {
        *vec.get(index).unwrap()
    }
}
