
use std::collections::BTreeMap;
use nom_supreme::ParserExt;
use num::integer::lcm;
use nom::{
    character::complete::{alphanumeric1, char},
    sequence::{separated_pair, terminated},
    bytes::complete::{tag, is_a},
    branch::alt,
    multi::many1,
    IResult,
    Parser,
};

#[derive(Debug, Clone)]
struct MapNode<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

pub fn part1<'a>(input: &'a str) -> u32 {
    let (moves_str, map) = input.split_once("\n\n").expect("Invalid map.");
    let (_, moves) = parse_moves(moves_str).expect("Invalid moves list.");
    let map: BTreeMap<&'a str, MapNode<'a>> = parse_map(map);

    // Find number of moves
    find_moves(moves, map, "AAA", "ZZZ")
}

pub fn part2<'a>(input: &'a str) -> usize {
    let (moves_str, map) = input.split_once("\n\n").expect("Invalid map.");
    let (_, moves) = parse_moves(moves_str).expect("Invalid moves list.");
    let map: BTreeMap<&'a str, MapNode<'a>> = parse_map(map);

    // Find number of moves
    find_ghost_moves(moves, map, "A", "Z")
}

fn find_moves<'a>(moves: Vec<Direction>, map: BTreeMap<&'a str, MapNode<'a>>, start: &str, end: &str) -> u32 {
    let mut steps: u32 = 0;
    let found: bool = false;

    let mut current = start;
    'upper: while !found {
        for step in moves.iter() {
            steps += 1;
            let node = map.get(current).expect("Node not found.");
            match *step {
                Direction::Right => {
                    current = node.right;
                    if node.right == end {
                        // Found it
                        break 'upper;
                    }
                },
                Direction::Left => {
                    current = node.left;
                    if node.left == end {
                        // Found it
                        break 'upper;
                    }
                },
            };
        }
    }
    steps
}

fn find_ghost_moves<'a>(moves: Vec<Direction>, map: BTreeMap<&'a str, MapNode<'a>>, start_end: &str, end_end: &str) -> usize {
    let starting_nodes: Vec<&'a str> = map
        .keys()
        .filter(|k| k.ends_with(start_end))
        .map(|k| *k)
        .collect();

    // Collect steps for each
    let steps: Vec<usize> = starting_nodes.iter().map(|start| {
        find_ghost_moves_single(&moves, &map, start, end_end)
    }).collect();

    if steps.len() >= 2 {
        return lcm_vec(&steps);
    }
    0
}

fn find_ghost_moves_single<'a>(moves: &Vec<Direction>, map: &BTreeMap<&'a str, MapNode<'a>>, start: &str, pattern: &str) -> usize {
    let found: bool = false;
    let mut steps: usize = 0;
    let mut current: &str = start;

    'upper: while !found {
        for the_move in moves.iter() {
            steps += 1;
            let node = map.get(current).expect("Node not found.");
            match the_move {
                Direction::Right => {
                    current = node.right;
                    if node.right.ends_with(pattern) {
                        break 'upper;
                    }
                },
                Direction::Left => {
                    current = node.left;
                    if node.left.ends_with(pattern) {
                        break 'upper;
                    }
                },
            }
        }
    }
    steps
}

fn parse_map<'a>(input: &'a str) -> BTreeMap<&'a str, MapNode<'a>> {
    let mut map: BTreeMap<&'a str, MapNode<'a>> = BTreeMap::new();
    for line in input.lines() {
        let (label, values) = parse_line(line).expect("Invalid instruction line.");
        let (_, (left, right)) = parse_instructions(values).unwrap();
        map.insert(label, MapNode { left, right });
    }

    map
}

fn parse_line(line: &str) -> Option<(&str, &str)> {
    line.split_once(" = ")
}

fn parse_moves(line: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        char('R').map(|_| Direction::Right),
        char('L').map(|_| Direction::Left),
    ))).parse(line)
}

fn parse_instructions(line: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(left_str, tag(", "), right_str).parse(line)
}

fn left_str(line: &str) -> IResult<&str, &str> {
    is_a("(").precedes(alphanumeric1).parse(line)
}

fn right_str(line: &str) -> IResult<&str, &str> {
    terminated(alphanumeric1, tag(")")).parse(line)
}

fn lcm_vec(input: &Vec<usize>) -> usize {
    if input.len() == 2 {
        return lcm(input[0], input[1]);
    } else {
        let first_num = input[0];
        let the_rest: Vec<usize> = input.iter().skip(1).map(|x| *x).collect();
        let second_num = lcm_vec(&the_rest);
        return lcm(first_num, second_num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        // Test final output
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_data2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        // Test final output
        let result = part1(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_lcm_2_items() {
        let input: Vec<usize> = vec![2, 3];
        let result = lcm_vec(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_lcm_4_items() {
        let input: Vec<usize> = vec![5, 10, 15, 25];
        let result = lcm_vec(&input);
        assert_eq!(result, 150);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        // Test final output
        let result = part2(input);
        assert_eq!(result, 6);
    }
}
