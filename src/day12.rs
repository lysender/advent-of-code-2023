
use nom::{
    multi::separated_list1,
    bytes::complete::is_a,
    IResult,
};
use indicatif::ProgressIterator;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct SpringRecord {
    conditions: String,
    stats: Vec<u32>,
}

pub fn part1(input: &str) -> u32 {
    let report = parse_input(input);
    report.iter().progress().map(|record| {
        get_arrangement_counts(record)
    }).sum()
}

pub fn part2(input: &str) -> u32 {
    let report = parse_input(input);
    // let report = parse_input_unfolded(input);
    report.iter().progress().map(|record| {
        get_arrangement_counts(record)
    }).sum()
}

fn compare_arrangement(record: &str, stats: &Vec<u32>) -> bool {
    // Check whether the record matches the given stats
    // Remove any outer "." character so that we can simply use a separated by list
    let line = record.trim_matches('.');
    if let Ok((_, damaged)) = parse_damaged(line) {
        if damaged.len() == stats.len() {
            let count: u32 = damaged.iter().zip(stats).map(|(str, stat)| {
                if str.len() == *stat as usize {
                    return 1;
                }
                return 0;
            }).sum();
            return count == damaged.len() as u32;
        }
    }
    false
}

fn get_arrangement_counts(record: &SpringRecord) -> u32 {
    // Find "?" positions
    let mut unknown_positions: Vec<usize> = Vec::new();
    for (i, ch) in record.conditions.chars().enumerate() {
        if ch == '?' {
            unknown_positions.push(i);
        }
    }

    let mut last_pos: Option<usize> = None;
    if unknown_positions.len() > 0 {
        last_pos = Some(unknown_positions.len() - 1);
    }


    get_arrangement_counts_inner(record.conditions.as_str(), &unknown_positions, last_pos, &record.stats)
}

fn get_arrangement_counts_inner_v2(conditions: &str, unknown_positions: &Vec<usize>, stats: &Vec<u32>) -> u32 {
    let mut matches_count: u32 = 0;

    for x in 0..unknown_positions.len() {
        for y in 0..unknown_positions.len() {
            // do nothing
        }
    }


    matches_count
}

fn get_arrangement_counts_inner(conditions: &str, unknown_positions: &Vec<usize>, last_pos: Option<usize>, stats: &Vec<u32>) -> u32 {
    // Replace each unknown condition with either "#" or "." from right to left
    // and check if it matches the stats.
    // Do this recursively and count 1 when it matches
    if let Some(pos) = last_pos {
        // Prevent overflow
        let mut next_pos = None;
        if pos > 0 {
            next_pos = Some(pos - 1);
        }

        let mut conditions_copy1 = conditions.to_string();
        let mut conditions_copy2 = conditions.to_string();

        let mut matches_count: u32 = 0;
        // Test for when it is damaged
        let index = unknown_positions[pos];
        conditions_copy1.replace_range(index..index+1, "#");
        matches_count += get_arrangement_counts_inner(conditions_copy1.as_str(), unknown_positions, next_pos, stats);

        // Test for when it is operational
        conditions_copy2.replace_range(index..index+1, ".");
        matches_count += get_arrangement_counts_inner(conditions_copy2.as_str(), unknown_positions, next_pos, stats);

        return matches_count;
    }

    // If there are no unknown positions, just test it if it matches the stats
    if compare_arrangement(conditions, stats) {
        return 1;
    }
    return 0;
}

fn get_arrangements_inner_v3(conditions: &str, stats: &Vec<u32>) -> u32 {
    // For each stat, find the lower and upper range of possible combinations
    // let r = the range
    // if there are no other stats to the right
    // min r = stat
    // otherwise
    // min r = stat + 1
    // max r is identified when
    // if are no damaged 
}

fn dots(line: &str) -> IResult<&str, &str> {
    is_a(".")(line)
}

fn sharps(line: &str) -> IResult<&str, &str> {
    is_a("#")(line)
}

fn parse_damaged(line: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(dots, sharps)(line)
}

fn parse_input(input: &str) -> Vec<SpringRecord> {
    input.lines().map(|line| {
        let (conditions, stats) = line.split_once(" ").expect("Should have two parts.");
        SpringRecord {
            conditions: conditions.to_string(),
            stats: stats.split(',').map(|x| {
                let num: u32 = x.to_string().parse::<u32>().expect("Stats should be numeric.");
                num
            }).collect(),
        }
    }).collect::<Vec<SpringRecord>>()
}

fn parse_input_unfolded(input: &str) -> Vec<SpringRecord> {
    input.lines().map(|line| {
        let (conditions, stats) = line.split_once(" ").expect("Should have two parts.");
        let mut unfolded_conditions: Vec<&str> = Vec::new();
        let mut unfolded_stats: Vec<&str> = Vec::new();

        for _ in 0..5 {
            unfolded_conditions.push(conditions);
            unfolded_stats.push(stats);
        }

        let u_conditions = unfolded_conditions.join("?");
        let u_stats = unfolded_stats.join(",");

        SpringRecord {
            conditions: u_conditions,
            stats: u_stats.split(',').map(|x| {
                let num: u32 = x.to_string().parse::<u32>().expect("Stats should be numeric.");
                num
            }).collect(),
        }
    }).collect::<Vec<SpringRecord>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_compare_arrangements() {
        assert!(compare_arrangement("#.#.###", &vec![1, 1, 3]));
        assert!(compare_arrangement(".#...#....###.", &vec![1, 1, 3]));
        assert!(compare_arrangement(".#.###.#.######", &vec![1, 3, 1, 6]));
        assert!(compare_arrangement("####.#...#...", &vec![4, 1, 1]));
        assert!(compare_arrangement("#....######..#####.", &vec![1, 6, 5]));
        assert!(compare_arrangement(".###.##....#", &vec![3, 2, 1]));
    }


    #[test]
    fn test_part1_arrangements() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let report = parse_input(input);
        assert_eq!(report.len(), 6);
        assert_eq!(get_arrangement_counts(&report[0]), 1);
        assert_eq!(get_arrangement_counts(&report[1]), 4);
        assert_eq!(get_arrangement_counts(&report[2]), 1);
        assert_eq!(get_arrangement_counts(&report[3]), 1);
        assert_eq!(get_arrangement_counts(&report[4]), 4);
        assert_eq!(get_arrangement_counts(&report[5]), 10);
    }

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = part1(input);
        assert_eq!(result, 21);
    }

    //#[test]
    fn test_part2_arrangements() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let report = parse_input_unfolded(input);
        assert_eq!(report.len(), 6);
        assert_eq!(get_arrangement_counts(&report[0]), 1);
        assert_eq!(get_arrangement_counts(&report[1]), 16384);
        assert_eq!(get_arrangement_counts(&report[2]), 1);
        assert_eq!(get_arrangement_counts(&report[3]), 4);
        assert_eq!(get_arrangement_counts(&report[4]), 2500);
        assert_eq!(get_arrangement_counts(&report[5]), 506250);
    }


    #[test]
    fn test_part2_combinations() {
        let symbols: Vec<char> = vec!['#', '.'];
        println!("About to test itertools::combinations");
        for row in (0..5).permutations(5) {
            println!("{:?}", row);
        }
        assert_eq!(1, 2);
    }

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = part2(input);
        assert_eq!(result, 525152);
    }
}
