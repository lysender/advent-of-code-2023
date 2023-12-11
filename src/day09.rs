
use nom::{
    character::complete::{self, space1, line_ending},
    multi::separated_list1,
    IResult,
    Parser,
};

pub fn part1(input: &str) -> i32 {
    let (_, histories) = parse_input(input).expect("Invalid histories input.");
    histories.iter().map(|history| {
        find_next_value(history)
    }).sum()
}

pub fn part2(input: &str) -> i32 {
    let (_, histories) = parse_input(input).expect("Invalid histories input.");
    histories.iter().map(|history| {
        find_prev_value(history)
    }).sum()
}

fn find_next_value(history: &Vec<i32>) -> i32 {
    // Find the bottom zeroes
    let mut current: Vec<i32> = history.clone();
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(history.clone());

    let mut is_bottom: bool = false;

    while !is_bottom {
        let mut next: Vec<i32> = Vec::new();
        for i in 1..current.len() {
            let a: i32 = current[i - 1];
            let b: i32 = current[i];
            let diff: i32 = b - a;
            next.push(diff);
        }

        sequences.push(next.clone());
        
        let sum: i32 = next.iter().sum();
        if sum == 0 {
            is_bottom = true;
        }

        current = next;
    }

    // Add 1 entry from the bottom then compute to the top
    let mut right: i32 = 0;
    for i in 1..sequences.len() {
        let index: usize = sequences.len() - (i + 1);
        let series = &sequences[index];
        let left: i32 = *series.last().expect("Series not be empty.");
        let new_right: i32 = right + left;
        right = new_right;
    }

    right
}

fn find_prev_value(history: &Vec<i32>) -> i32 {
    // Find the bottom zeroes
    let mut current: Vec<i32> = history.clone();
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(history.clone());

    let mut is_bottom: bool = false;

    while !is_bottom {
        let mut next: Vec<i32> = Vec::new();
        for i in 1..current.len() {
            let a: i32 = current[i - 1];
            let b: i32 = current[i];
            let diff: i32 = b - a;
            next.push(diff);
        }

        sequences.push(next.clone());
        
        let sum: i32 = next.iter().sum();
        if sum == 0 {
            is_bottom = true;
        }

        current = next;
    }

    // Add 1 entry from the bottom then compute to the top
    let mut left: i32 = 0;
    for i in 1..sequences.len() {
        let index: usize = sequences.len() - (i + 1);
        let series = &sequences[index];
        let right: i32 = *series.first().expect("Series not be empty.");
        let new_left: i32 = right - left;
        left = new_left;
    }

    left
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(line: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32).parse(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        // Test final output
        let result = part1(input);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        // Test final output
        let result = part2(input);
        assert_eq!(result, 2);
    }
}
