use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let reader = BufReader::new(get_file());
    let mut total_priority: u32 = 0;

    for (line_index, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();

        if is_valid_line(&line_str) {
            let mut repeated_types: Vec<char> = Vec::new();
            // Leave 0 index unused for simplicity
            let mut priorities = [0; 53];

            let compartment_size = line_str.len() / 2;

            for (i, char) in line_str.chars().enumerate() {
                // usize to make array happy, u32 is compatible  (on x86-64 at least, usize bigger, 64 bit)
                let priority = calculate_priority(char);
                let priority_as_usize = priority as usize;
                // First compartment
                if i < compartment_size {
                    priorities[priority_as_usize] += 1;
                } else {
                    if priorities[priority_as_usize] > 0 {
                        if !repeated_types.contains(&char) {
                            repeated_types.push(char);
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

// TODO create Priority type with internal methods for conversions
fn calculate_priority(item_type: char) -> u32 {
    // internal values: a = 97, A = 65
    return match item_type.is_lowercase() {
        true => ((item_type as u32) - 97) + 1,
        false => ((item_type as u32) - 65) + 27,
    };
}

fn get_file() -> File {
    let home_dir = env::var("HOME").unwrap();
    let path = "github/adventofcode-2022/day_3/src";
    let filename = "rucksack_items.log";
    let full_path = format!("{}/{}/{}", home_dir, path, filename);
    File::open(full_path).unwrap()
}


#[cfg(test)]
mod tests {
    use crate::calculate_priority;

    #[test]
    fn calculate_priority_for_a() {
        let actual = calculate_priority('a');
        assert_eq!(actual, 1);
    }

    #[test]
    fn calculate_priority_for_z() {
        let actual = calculate_priority('z');
        assert_eq!(actual, 26);
    }

    #[test]
    fn calculate_priority_for_A() {
        let actual = calculate_priority('A');
        assert_eq!(actual, 27);
    }

    #[test]
    fn calculate_priority_for_Z() {
        let actual = calculate_priority('Z');
        assert_eq!(actual, 52);
    }
}
