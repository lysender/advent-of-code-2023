
use std::{fs, path::PathBuf};
use std::path::Path;

use aoc2023::day02::{part1, part2, CubeSet};

fn main() {
    run_part1();
    run_part2();
}

pub fn run_part1() {
    let filename: PathBuf = Path::new("data").join("day02-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let cube_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let value = part1(input_string.as_str(), cube_set);
    println!("Total games: {}", value);
}

pub fn run_part2() {
    let filename: PathBuf = Path::new("data").join("day02-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = part2(input_string.as_str());
    println!("Sum of min power: {}", value);
}
