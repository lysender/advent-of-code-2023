
use std::{fs, path::PathBuf};
use std::path::Path;

use aoc2023::day02::{part1, part2, CubeSet};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    let filename: PathBuf = Path::new("data").join("day02-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let cube_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    part1(
        divan::black_box(input_string.as_str()),
        divan::black_box(cube_set),
    );
}

#[divan::bench]
fn part2_bench() {
    let filename: PathBuf = Path::new("data").join("day02-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    part2(divan::black_box(input_string.as_str()));
}
