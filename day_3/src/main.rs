use std::fs::File;
use std::io::{BufRead, BufReader};

use file_utils::read_file;

// https://adventofcode.com/2022/day/3

fn main() {
    let total_priority = find_repeated_types();
    println!("Total priority for all duplicated items: {}", total_priority);
}

// TODO line length must be even
fn is_valid_line(line: &String) -> bool {
    line.len() > 0 && !line.starts_with("//")
}

fn find_repeated_types() -> u32 {
    let reader = read_file();
    let mut total_priority: u32 = 0;

    for (line_index, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();

        if is_valid_line(&line_str) {
            let mut repeated_types: Vec<Item> = Vec::new();
            // Leave 0 index unused for simplicity
            let mut priorities = [0; 53];

            let compartment_size = line_str.len() / 2;

            for (i, char) in line_str.chars().enumerate() {
                let item = Item::from(char);
                let priority = item.calculate_priority();
                // usize to make array happy, u32 is compatible  (on x86-64 at least, usize bigger, 64 bit)
                let priority_as_usize = priority as usize;
                // First compartment
                if i < compartment_size {
                    priorities[priority_as_usize] += 1;
                } else {
                    if priorities[priority_as_usize] > 0 {
                        if !repeated_types.contains(&item) {
                            repeated_types.push(item);
                            total_priority += priority;
                        }
                        priorities[priority_as_usize] += 1;
                    }
                }
            }

            // Count from 0 as normal people do
            println!("Repeated elements for line {}: {:?}", line_index + 1, repeated_types)
        }
    }

    total_priority
}

#[derive(Debug)]
struct Item {
    priority: char,
}

impl Item {
    fn from(priority: char) -> Item {
        Item { priority }
    }

    fn calculate_priority(&self) -> u32 {
        // internal values: a = 97, A = 65
        return match self.priority.is_lowercase() {
            true => ((self.priority as u32) - 97) + 1,
            false => ((self.priority as u32) - 65) + 27,
        };
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }

    fn ne(&self, other: &Self) -> bool {
        self.priority != other.priority
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
        let path = "github/adventofcode-2022/day_3/src";
        let filename = "rucksack_items.log";
        let full_path = format!("{}/{}/{}", home_dir, path, filename);
        File::open(full_path).unwrap()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod item_tests {
    use crate::Item;

    #[test]
    fn calculate_priority_for_a() {
        let item = Item::from('a');
        let actual = item.calculate_priority();
        assert_eq!(actual, 1);
    }

    #[test]
    fn calculate_priority_for_z() {
        let item = Item::from('z');
        let actual = item.calculate_priority();
        assert_eq!(actual, 26);
    }

    #[test]
    fn calculate_priority_for_A() {
        let item = Item::from('A');
        let actual = item.calculate_priority();
        assert_eq!(actual, 27);
    }

    #[test]
    fn calculate_priority_for_Z() {
        let item = Item::from('Z');
        let actual = item.calculate_priority();
        assert_eq!(actual, 52);
    }
}
