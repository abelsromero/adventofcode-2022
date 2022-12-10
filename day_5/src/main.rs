use std::collections::{HashMap, VecDeque};
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

use file_utils::read_file;

lazy_static! {
    static ref MOVEMENT_PATTERN: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    static ref STACK_NUMBERS_PATTERN: Regex = Regex::new(r"^\s*\d+(?:\s+\d+)*\s*$").unwrap();
}

fn main() {
    operate_crane();
}

fn is_movement_line(line: &str) -> bool {
    MOVEMENT_PATTERN.is_match(line) || is_comment(line)
}

fn is_comment(line: &str) -> bool {
    line.starts_with("//")
}

fn operate_crane() {
    let reader = read_file();

    let mut movements: VecDeque<Movement> = VecDeque::new();

    let lines: Vec<String> = reader.lines()
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
        let stacks_numbers = parse_stack_numbers(&line_str);
        if stacks_numbers.len() == 0 {
            println!("No valid stack numbers found");
            return;
        }

        let mut stacks = stacks_numbers.iter()
            .map(|v| (*v, Stack::new()))
            .collect::<HashMap<u32, Stack>>();

        let x = stacks.get_mut(&0).unwrap();
        x.push("X");
        //x.crates.push("hullo");
        println!("{}", 2);

        //parse_stack_crates(&line_str, &stacks);
    }
}

fn parse_stack_numbers(line: &str) -> Vec<u32> {
    let mut stack_numbers = Vec::new();

    if STACK_NUMBERS_PATTERN.is_match(&line) {
        // Cannot use groups when the number of elements is not known, so "old school"
        let mut matching: bool = false;
        let mut start_match: usize = 0;
        for (i, char) in line.chars().enumerate() {
            if matching {
                if char.is_whitespace() {
                    let number_value: u32 = line[start_match..i].parse().unwrap();
                    stack_numbers.push(number_value);
                    matching = false;
                } else {
                    continue;
                }
            }
            if !matching {
                if char.is_whitespace() {
                    continue;
                } else {
                    matching = true;
                    start_match = i;
                }
            }
        }
        if matching {
            let number_value: u32 = line[start_match..line.len()].parse().unwrap();
            stack_numbers.push(number_value);
        }
    }
    stack_numbers
}

fn parse_stack_crates(line: &str, stacks: &mut HashMap<u32, Stack>) {
    let mut matching: bool = false;
    let mut start_match: usize = 0;

    let mut crate_index = 0;

    for (i, char) in line.chars().enumerate() {
        if matching {
            if char == ']' {
                let crate_type = &line[start_match..i - 1];
                let x = stacks.get_mut(&crate_index).unwrap();
                x.push(crate_type);
                matching = false;
                crate_index += 1;
            } else {
                continue;
            }
        }
        if !matching {
            if char == '[' {
                matching = true;
                start_match = i;
            } else {
                continue;
            }
        }
    }
    if matching {
        let crate_type = &line[start_match..line.len() - 1];
        stacks.get_mut(&crate_index).unwrap()
            .push(crate_type);
    }
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

struct Stack<'a> {
    crates: Vec<&'a str>,
}

impl Stack<'_> {
    fn new() -> Stack {
        Stack { crates: Vec::new() }
    }

    fn len(&self) -> usize {
        self.crates.len()
    }

    pub fn push(&mut self, value: &str) {
        self.crates.push(value)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
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
        let line = "    11    232     323 454    565  676";
        let vec = parse_stack_numbers(line);
        assert_eq!(vec.len(), 6);
        assert_eq!(value_at(&vec, 0), 11);
        assert_eq!(value_at(&vec, 1), 232);
        assert_eq!(value_at(&vec, 2), 323);
        assert_eq!(value_at(&vec, 3), 454);
        assert_eq!(value_at(&vec, 4), 565);
        assert_eq!(value_at(&vec, 5), 676);
    }

    fn value_at(vec: &Vec<u32>, index: usize) -> u32 {
        *vec.get(index).unwrap()
    }
}

#[cfg(test)]
mod stack_crates_parser {
    use std::collections::HashMap;

    use crate::Stack;

    #[test]
    fn parse_single_crate() {
        let line = "[A]";
        let stacks = HashMap::from([(1, Stack::new())]);
        // parse_stack_crates(line, &stacks);
        assert_eq!(stacks.get(&1).unwrap().len(), 1)
    }
}
