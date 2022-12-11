use std::borrow::BorrowMut;
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

        let mut stacks_pool = StacksPool::from(stacks_numbers);
        parse_stack_crates(&line_str, stacks_pool.borrow_mut());

        println!("Processed stacks");
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

fn parse_stack_crates<'a>(line: &'a str, stacks: &mut StacksPool<'a>) {
    let mut matching: bool = false;
    let mut start_match: usize = 0;

    // Deference the reference to avoid borrowing issues, this could be a smell?
    let mut stack_index = 0;

    for (i, char) in line.chars().enumerate() {
        if matching {
            if char == ']' {
                let crate_type = &line[start_match + 1..i];
                let stack_id = *stacks.ids.get(stack_index).unwrap();
                let stack = stacks.get_mut(stack_id).unwrap();
                stack.push(crate_type);
                matching = false;
                stack_index += 1;
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
        let stack_id = *stacks.ids.get(stack_index).unwrap();
        stacks.get_mut(stack_id).unwrap()
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

// Keep copy of ids because HashMap::keys does not maintain insert order
// with ::insert or ::from
struct StacksPool<'a> {
    ids: Vec<u32>,
    stacks: HashMap<u32, Stack<'a>>,
}

impl<'a> StacksPool<'a> {
    // TODO can we use array to avoid move issues with Vector and since we know the size in advance?
    fn from(ids: Vec<u32>) -> StacksPool<'a> {
        let mut stacks = HashMap::new();
        // clone bc: 'move occurs because `ids` has type `Vec<u32>`, which does not implement the `Copy` trait'
        for id in ids.clone() {
            stacks.insert(id, Stack::new());
        }

        StacksPool { ids, stacks }
    }

    // TODO can we make it to return non-reference?
    // In theory a returned value already is
    pub fn get(&self, pool_id: u32) -> Option<&Stack<'a>> {
        self.stacks.get(&pool_id)
    }

    pub fn get_mut(&mut self, pool_id: u32) -> Option<&mut Stack<'a>> {
        self.stacks.get_mut(&pool_id)
    }
}

struct Stack<'a> {
    crates: Vec<&'a str>,
}

impl<'a> Stack<'a> {
    fn new() -> Stack<'a> {
        Stack { crates: Vec::new() }
    }

    fn len(&self) -> usize {
        self.crates.len()
    }

    pub fn push(&mut self, value: &'a str) {
        // This requires to set the lifetime because the &str reference
        // could be out of scope.
        // Then, this requires the lifetime definition to be propagated
        // to the imp, struct and 'parse_stack_crates' method.
        self.crates.push(value)
    }
}

// TODO test, does this work without
// struct StringStack {
//     crates: Vec<String>,
// }
//
// impl StringStack {
//     fn new() -> StringStack {
//         StringStack { crates: Vec::new() }
//     }
//
//     fn len(&self) -> usize {
//         self.crates.len()
//     }
//
//     pub fn push(&mut self, value: String) {
//         self.crates.push(value)
//     }
// }

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
    use std::borrow::BorrowMut;
    use std::collections::HashMap;

    use crate::{parse_stack_crates, Stack, StacksPool};

    #[test]
    fn parse_single_crate() {
        let line = "[A]";
        let stack_id = 1u32;
        let mut stacks = StacksPool::from(Vec::from([stack_id]));

        parse_stack_crates(line, stacks.borrow_mut());

        let stack = stacks.get(stack_id).unwrap();
        assert_eq!(stack.len(), 1);
        assert_eq!(*stack.crates.get(0).unwrap(), "A");
    }

    #[test]
    fn parse_three_crates() {
        let line = "[A] [B] [C]";
        let mut stacks = StacksPool::from(Vec::from([1, 2, 3]));

        parse_stack_crates(line, stacks.borrow_mut());

        let first_stack = stacks.get(1).unwrap();

        assert_eq!(first_stack.len(), 1);
        assert_eq!(*first_stack.crates.get(0).unwrap(), "A");

        let second_stack = stacks.get(2).unwrap();
        assert_eq!(second_stack.len(), 1);
        assert_eq!(*second_stack.crates.get(0).unwrap(), "B");

        let third_stack = stacks.get(3).unwrap();
        assert_eq!(third_stack.len(), 1);
        assert_eq!(*third_stack.crates.get(0).unwrap(), "C");
    }

    #[test]
    fn parse_complex_crates() {
        let line = "   [Y]   [Z] [X]   ";
        let mut stacks = StacksPool::from(Vec::from([11, 22, 33]));

        parse_stack_crates(line, stacks.borrow_mut());

        let first_stack = stacks.get(11).unwrap();
        assert_eq!(first_stack.len(), 1);
        assert_eq!(*first_stack.crates.get(0).unwrap(), "Y");

        let second_stack = stacks.get(22).unwrap();
        assert_eq!(second_stack.len(), 1);
        assert_eq!(*second_stack.crates.get(0).unwrap(), "Z");

        let third_stack = stacks.get(33).unwrap();
        assert_eq!(third_stack.len(), 1);
        assert_eq!(*third_stack.crates.get(0).unwrap(), "X");
    }
}
