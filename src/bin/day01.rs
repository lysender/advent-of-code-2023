use std::{fs, path::PathBuf};
use std::path::Path;

use aoc2023::day01::{part1, part2};

fn main() {
    run_part1();
    run_part2();
}

pub fn run_part1() {
    let filename: PathBuf = Path::new("data").join("day01-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = part1(input_string.as_str());
    println!("Calibration value total: v1: {}", value);
}

pub fn run_part2() {
    let filename: PathBuf = Path::new("data").join("day01-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = part2(input_string.as_str());

    println!("Calibration value total: v2: {}", value);
}

