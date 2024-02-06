use nom::{
    multi::separated_list1,
    bytes::complete::is_a,
    IResult,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use indicatif::ProgressIterator;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct SpringRecord {
    conditions: String,
    stats: Vec<u32>,
}

#[derive(Hash)]
struct Payload<'a> {
    conditions: &'a str,
    last_pos: &'a Option<usize>,
}

struct MemoCompute {
    cache: HashMap<u64, u32>,
    unknown_positions: Vec<usize>,
    stats: Vec<u32>,
}

impl MemoCompute {
    fn new(record: &SpringRecord) -> Self {
        // Find "?" positions
        let mut unknown_positions: Vec<usize> = Vec::new();
        for (i, ch) in record.conditions.chars().enumerate() {
            if ch == '?' {
                unknown_positions.push(i);
            }
        }

        MemoCompute {
            cache: HashMap::new(),
            unknown_positions,
            stats: record.stats.clone(),
        }
    }

    fn count_arrangements(&mut self, conditions: &str, last_pos: Option<usize>) -> u32 {
        // Replace each unknown condition with either "#" or "." from right to left
        // and check if it matches the stats.
        // Do this recursively and count 1 when it matches
        if let Some(pos) = last_pos {
            let index = self.unknown_positions[pos];

            // Prevent overflow
            let mut next_pos = None;
            if pos > 0 {
                next_pos = Some(pos - 1);
            }

            let mut conditions_copy1 = conditions.to_string();
            let mut conditions_copy2 = conditions.to_string();

            // Test for when it is damaged
            conditions_copy1.replace_range(index..index+1, "#");

            let def_count: u32;
            let def_tail = &conditions_copy1[index..];
            let def_cache_key = create_payload_key(def_tail, &next_pos);
            if let Some(def_count_cached) = self.cache.get(&def_cache_key) {
                println!("cache hit... {} = {}", def_cache_key, def_count_cached);
                def_count = *def_count_cached;
            } else {
                def_count = self.count_arrangements(conditions_copy1.as_str(),next_pos);
                println!("setting cache: {} = {}, {}", def_cache_key, def_count, def_tail);
                self.cache.insert(def_cache_key, def_count);
            }

            // Test for when it is operational
            conditions_copy2.replace_range(index..index+1, ".");

            let working_count: u32;
            let working_tail = &conditions_copy2[index..];
            let working_cache_key = create_payload_key(working_tail, &next_pos);
            if let Some(working_count_cached) = self.cache.get(&working_cache_key) {
                println!("cache hit... {} = {}", working_cache_key, working_count_cached);
                working_count = *working_count_cached;
            } else {
                working_count = self.count_arrangements(conditions_copy2.as_str(), next_pos);
                self.cache.insert(working_cache_key, working_count);
            }

            return def_count + working_count;
        }

        // If there are no unknown positions, just test it if it matches the stats
        if compare_arrangement(conditions, &self.stats) {
            return 1;
        }
        return 0;
    }
}

pub fn part1(input: &str) -> u32 {
    let report = parse_input(input);
    report.iter().progress().map(|record| {
        get_arrangement_counts_memo(record)
    }).sum()
}

pub fn part2(input: &str) -> u32 {
    let report = parse_input(input);
    report.iter().progress().map(|record| {
        get_arrangement_counts_memo(record)
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

    // Create a caching mechanism for partial calculations
    // Find a way to pass it around and allow immutable read and mutable write
    let cache: Rc<RefCell<HashMap<u64, u32>>> = Rc::new(RefCell::new(HashMap::new()));

    get_arrangement_counts_inner(&cache, record.conditions.as_str(), &unknown_positions, last_pos, &record.stats)
}

fn get_arrangement_counts_memo(record: &SpringRecord) -> u32 {
    let conditions = record.conditions.clone();
    let mut memo = MemoCompute::new(record);
    let mut last_pos: Option<usize> = None;
    if memo.unknown_positions.len() > 0 {
        last_pos = Some(memo.unknown_positions.len() - 1);
    }

    memo.count_arrangements(&conditions, last_pos)
}

fn get_arrangement_counts_inner(
    cache: &Rc<RefCell<HashMap<u64, u32>>>,
    conditions: &str,
    unknown_positions: &Vec<usize>,
    last_pos: Option<usize>,
    stats: &Vec<u32>,
) -> u32 {
    // Replace each unknown condition with either "#" or "." from right to left
    // and check if it matches the stats.
    // Do this recursively and count 1 when it matches
    if let Some(pos) = last_pos {
        let index = unknown_positions[pos];

        // Prevent overflow
        let mut next_pos = None;
        if pos > 0 {
            next_pos = Some(pos - 1);
        }

        let mut conditions_copy1 = conditions.to_string();
        let mut conditions_copy2 = conditions.to_string();

        // Test for when it is damaged
        conditions_copy1.replace_range(index..index+1, "#");

        let def_count: u32;
        let def_tail = &conditions_copy1[index..];
        let def_cache_key = create_payload_key(def_tail, &next_pos);
        if let Some(def_count_cached) = cache.borrow().get(&def_cache_key) {
            def_count = *def_count_cached;
        } else {
            def_count = get_arrangement_counts_inner(cache, conditions_copy1.as_str(), unknown_positions, next_pos, stats);
            cache.borrow_mut().insert(def_cache_key, def_count);
        }

        // Test for when it is operational
        conditions_copy2.replace_range(index..index+1, ".");

        let working_count: u32;
        let working_tail = &conditions_copy2[index..];
        let working_cache_key = create_payload_key(working_tail, &next_pos);
        if let Some(working_count_cached) = cache.borrow().get(&working_cache_key) {
            working_count = *working_count_cached;
        } else {
            working_count = get_arrangement_counts_inner(cache, conditions_copy2.as_str(), unknown_positions, next_pos, stats);
            cache.borrow_mut().insert(working_cache_key, working_count);
        }

        return def_count + working_count;
    }

    // If there are no unknown positions, just test it if it matches the stats
    if compare_arrangement(conditions, stats) {
        return 1;
    }
    return 0;
}

fn create_payload_key(
    conditions: &str,
    last_pos: &Option<usize>,
) -> u64 {
    let mut usher = DefaultHasher::new();
    let payload = Payload {
        conditions,
        last_pos,
    };

    payload.hash(&mut usher);
    usher.finish()
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
        assert_eq!(get_arrangement_counts_memo(&report[0]), 1);
        assert_eq!(get_arrangement_counts_memo(&report[1]), 4);
        assert_eq!(get_arrangement_counts_memo(&report[2]), 1);
        assert_eq!(get_arrangement_counts_memo(&report[3]), 1);
        assert_eq!(get_arrangement_counts_memo(&report[4]), 4);
        assert_eq!(get_arrangement_counts_memo(&report[5]), 10);
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
        assert_eq!(get_arrangement_counts_memo(&report[0]), 1);
        assert_eq!(get_arrangement_counts_memo(&report[1]), 16384);
        assert_eq!(get_arrangement_counts_memo(&report[2]), 1);
        assert_eq!(get_arrangement_counts_memo(&report[3]), 4);
        assert_eq!(get_arrangement_counts_memo(&report[4]), 2500);
        assert_eq!(get_arrangement_counts_memo(&report[5]), 506250);
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
