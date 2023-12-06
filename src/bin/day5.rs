
use std::time::Instant;
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
    let ts = Instant::now();
    let value = part1(input_string.as_str());
    let duration = ts.elapsed().as_millis();
    println!("Closest location: {}", value);
    println!("Duration: {} ms", duration);
}

pub fn run_part2() {
    let filename: PathBuf = Path::new("data").join("day5-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let ts = Instant::now();
    let value = part2(input_string.as_str());
    let duration = ts.elapsed().as_millis();
    println!("Closest location v2: {}", value);
    println!("Duration: {} ms", duration);
}
