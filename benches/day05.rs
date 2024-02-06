use std::{fs, path::PathBuf};
use std::path::Path;

use aoc2023::day05::part1;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    let filename: PathBuf = Path::new("data").join("day05-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    part1(divan::black_box(input_string.as_str()));
}
