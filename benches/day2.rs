
use std::{fs, path::PathBuf};
use std::path::Path;

use aoc2023::day2;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    let filename: PathBuf = Path::new("data").join("day2-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let cube_set = day2::CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    day2::part1(
        divan::black_box(input_string.as_str()),
        divan::black_box(cube_set),
    );
}

#[divan::bench]
fn part2_bench() {
    let filename: PathBuf = Path::new("data").join("day2-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    day2::part2(divan::black_box(input_string.as_str()));
}
