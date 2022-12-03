#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

// https://adventofcode.com/2022/day/2

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSOR_SCORE: i32 = 3;

const LOST_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

lazy_static! {
    static ref LINE_PATTERN: Regex = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
}

fn main() {
    let score: i32 = calculate_total_score();

    println!("Your score is: {}", score)
}

fn calculate_total_score() -> i32 {
    let reader = BufReader::new(get_file());

    let mut total_score = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        if is_valid_line(&line_str) {
            let captures = LINE_PATTERN.captures(&line_str).unwrap();
            // no need for error handling, we know line matches
            let my_play = captures.get(2).unwrap().as_str();
            let opponent_play = captures.get(1).unwrap().as_str();
            let score: i32 = calculate_win(opponent_play, my_play);
            let hand_score = calculate_hand_score(my_play) + score;
            println!("Processed line {} for {}", line_str, hand_score);
            total_score += hand_score;
        }
    }

    total_score
}

fn calculate_hand_score(my_play: &str) -> i32 {
    // We know only possible values are X, Y, Z thanks to regex validation
    match my_play {
        MY_ROCK => 1,
        MY_PAPER => 2,
        _ => 3,
    }
}

fn calculate_win(opponent_play: &str, my_play: &str) -> i32 {
    if (my_play == MY_ROCK && opponent_play == OPPONENT_ROCK)
        || (my_play == MY_PAPER && opponent_play == OPPONENT_PAPER)
        || (my_play == MY_SCISSORS && opponent_play == OPPONENT_SCISSORS) {
        return DRAW_SCORE;
    }
    if (my_play == MY_ROCK && opponent_play == OPPONENT_SCISSORS)
        || (my_play == MY_PAPER && opponent_play == OPPONENT_ROCK)
        || (my_play == MY_SCISSORS && opponent_play == OPPONENT_PAPER) {
        return WIN_SCORE;
    }

    LOST_SCORE
}

fn is_valid_line(line: &String) -> bool {
    LINE_PATTERN.is_match(line)
}

fn get_file() -> File {
    let home_dir = env::var("HOME").unwrap();
    let path = "github/adventofcode-2022/day_2/src";
    let filename = "strategy_guide.log";
    let full_path = format!("{}/{}/{}", home_dir, path, filename);
    File::open(full_path).unwrap()
}
