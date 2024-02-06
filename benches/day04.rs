use std::{fs, path::PathBuf};
use std::path::Path;

use aoc2023::day04::{part1, part2};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    let filename: PathBuf = Path::new("data").join("day04-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    part1(divan::black_box(input_string.as_str()));
}

#[divan::bench]
fn part2_bench() {
    let filename: PathBuf = Path::new("data").join("day04-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    part2(divan::black_box(input_string.as_str()));
}
