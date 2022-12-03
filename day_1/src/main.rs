use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = get_file();
    let reader = BufReader::new(file);

    let mut elfs: Vec<i32> = Vec::new();
    let mut current_elf = 0;
    // init first position
    elfs.push(0);

    for line in reader.lines() {
        let line_str = line.unwrap();
        // println!("Processing: {}", line_str);
        if line_str.len() == 0 {
            current_elf += 1;
            elfs.push(0);
            // println!("New elf! {}", current_elf)
        } else {
            // TODO handle parsing errors
            let x: i32 = line_str.parse().unwrap();
            elfs[current_elf] += x;
            // println!("Update! {:?}", elfs);
        }
    }

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
        println!("Elf {} is one, with {} calories", max_elf_id, max_calories);
    }
}

fn get_file() -> File {
    let home_dir = env::var("HOME").unwrap();
    let path = "github/adventofcode-2022/day_1/src";
    let calories_file = "calories.log";
    let string = format!("{}/{}/{}", home_dir, path, calories_file);
    File::open(string).unwrap()
}
