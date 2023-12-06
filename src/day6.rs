
use nom::bytes::complete::{tag, take_till};
use nom::IResult;

pub fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() != 2 {
        return 0;
    }

    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    if let Ok(time_line) = parse_data_line(lines[0], "Time:") {
        times = parse_number_line(time_line.0.trim());
    }
    if let Ok(distance_line) = parse_data_line(lines[1], "Distance:") {
        distances = parse_number_line(distance_line.0.trim());
    }

    if times.len() == distances.len() {
        let ways: Vec<u64> = times.iter().enumerate().map(|(i, time)| {
            let distance = distances[i];
            get_ways_to_win(*time, distance)
        }).collect();

        let result: u64 = ways.iter().product();
        return result
    }

    0
}

pub fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() != 2 {
        return 0;
    }

    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    if let Ok(time_line) = parse_data_line(lines[0], "Time:") {
        let time_str = time_line.0.trim().replace(" ", "").to_string();
        time = time_str.parse::<u64>().unwrap();
    }
    if let Ok(distance_line) = parse_data_line(lines[1], "Distance:") {
        let distance_str = distance_line.0.trim().replace(" ", "").to_string();
        distance = distance_str.parse::<u64>().unwrap();
    }

    get_ways_to_win(time, distance)
}

fn parse_data_line<'a >(line: &'a str, prefix: &'a str) -> IResult<&'a str, &'a str> {
    tag(prefix)(line)
}

fn parse_number_line(line: &str) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut remainder: &str = line;
    while remainder.len() > 0 {
        if let Ok(parsed_numbers) = parse_numbers(remainder) {
            if parsed_numbers.1.len() > 0 {
                let num_str = parsed_numbers.1.to_string();
                let num: u64 = num_str.parse::<u64>().unwrap();
                result.push(num);
            }

            remainder = parsed_numbers.0.trim();
        } else {
            break;
        }
    }
    result
}

fn parse_numbers(line: &str) -> IResult<&str, &str> {
    take_till(|c| c == ' ')(line)
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
