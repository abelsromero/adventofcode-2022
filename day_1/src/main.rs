use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// https://adventofcode.com/2022/day/1

fn main() {
    let elfs: Vec<i32> = process_calories();

    if elfs.is_empty() {
        println!("Sorry, no food today :(")
    } else {
        let mut max_elf_id = 0;
        let mut max_calories = elfs[0];
        for (elf, calories) in elfs.iter().enumerate() {
            if *calories >= max_calories {
                max_elf_id = elf;
                max_calories = *calories;
            }
        }
        // Count from 1, as normal ppl do
        println!("Elf {} is one, with {} calories", max_elf_id + 1, max_calories);
    }
}

fn process_calories() -> Vec<i32> {
    let reader = BufReader::new(get_file());

    let mut elfs = Vec::new();
    let mut current_elf = 0;
    // init first position
    elfs.push(0);

    for line in reader.lines() {
        let line_str = line.unwrap();
        // println!("Processing: {}", line_str);
        // Do not add a new elf for every empty line
        if line_str.len() == 0 && elfs[current_elf] > 0 {
            current_elf += 1;
            elfs.push(0);
            // println!("New elf! {}", current_elf)
        } else {
            let result: Result<i32, _> = line_str.parse();
            if result.is_ok() {
                elfs[current_elf] += result.unwrap();
            }
            // println!("Update! {:?}", elfs);
        }
    }

    if elfs.iter().all(|v| *v == 0) {
        Vec::new()
    } else {
        elfs
    }
}

fn get_file() -> File {
    let home_dir = env::var("HOME").unwrap();
    let path = "github/adventofcode-2022/day_1/src";
    let calories_file = "calories.log";
    let string = format!("{}/{}/{}", home_dir, path, calories_file);
    File::open(string).unwrap()
}
