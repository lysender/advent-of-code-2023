
use std::{fs, path::PathBuf};
use std::path::Path;

use aoc2023::day5::{part1, part2};

fn main() {
    run_part1();
    run_part2();
}

pub fn run_part1() {
    let filename: PathBuf = Path::new("data").join("day5-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = part1(input_string.as_str());
    println!("Closest location: {}", value);
}

pub fn run_part2() {
    let filename: PathBuf = Path::new("data").join("day5-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    // let value = part2(input_string.as_str());
    // println!("closest location v2: {}", value);
}
