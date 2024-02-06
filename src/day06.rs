use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1, digit1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

pub fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() != 2 {
        return 0;
    }

    if let Ok((_, numbers)) = parse_input(input) {
        let times = numbers.0;
        let distances = numbers.1;

        let ways: u64 = times.iter().zip(distances).map(|(time, distance)| {
            get_ways_to_win(*time, distance)
        }).product();

        return ways;
    }

    0
}

pub fn part2(input: &str) -> u64 {
    if let Ok((_, numbers)) = parse_input2(input) {
        let time = numbers.0;
        let distance = numbers.1;
        return get_ways_to_win(time, distance);
    }
    return 0;
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    // Collect two parsed result separated by new line
    // Each parser will then return a vec of u64, defined below
    separated_pair(collect_numbers, line_ending, collect_numbers).parse(input)
}

fn collect_numbers(line: &str) -> IResult<&str, Vec<u64>> {
    // Match anything that is not numeric, then throw it away
    // Feed the remainder to the next
    // Parse remainder separated by 1 least 1 space
    // Expect at least 1 result
    // Convert match to u64, resulting in a vec of u64
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u64))
        .parse(line)
}

fn parse_input2(input: &str) -> IResult<&str, (u64, u64)> {
    // Collect two parsed result separated by new line
    // Each parser will return a u64 value, defined below
    separated_pair(extract_number, line_ending, extract_number).parse(input)
}

fn extract_number(line: &str) -> IResult<&str, u64> {
    // Match anything that is not numeric, then throw it away
    // Feed the remainder to the next
    // Parse remainder separated by at least 1 space
    // but join then together and parse manually into u64
    // Expects at least 1 result
    is_not("0123456789")
        .precedes(
            separated_list1(space1, digit1).map(|list| {
                list.join("")
                    .parse::<u64>()
                    .expect("a valid number")
            })
        ).parse(line)
}

// Hold for hold milliseconds and returns the distance run in millimeters
// max_time constraint where race only last max_time in milliseconds
fn run_race(hold: u64, max_time: u64) -> u64 {
    if hold >= max_time {
        return 0;
    }

    if hold == 0 {
        return 0;
    }

    (max_time - hold) * hold
}

fn get_ways_to_win(max_time: u64, to_beat: u64) -> u64{
    let mut ways: u64 = 0;
    for hold in 1..max_time {
        let distance = run_race(hold, max_time);
        if distance > to_beat {
            ways += 1;
        }
    }
    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_race() {
        assert_eq!(run_race(0, 7), 0);
        assert_eq!(run_race(1, 7), 6);
        assert_eq!(run_race(2, 7), 10);
        assert_eq!(run_race(3, 7), 12);
        assert_eq!(run_race(4, 7), 12);
        assert_eq!(run_race(5, 7), 10);
        assert_eq!(run_race(6, 7), 6);
        assert_eq!(run_race(7, 7), 0);
    }

    #[test]
    fn test_get_ways_to_win() {
        assert_eq!(get_ways_to_win(7, 9), 4);
    }

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        // Test final output
        let result = part1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = part2(input);
        assert_eq!(result, 71503);
    }
}
