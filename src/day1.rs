
use std::{fs, path::PathBuf};
use std::path::Path;

const SPELLED_DIGITS: [&'static str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

pub fn day01_puzzle01() {
    // Input as string with multiple lines
    // Each line may contain digits, combine the first and last digit to form a single
    // two-digit number
    // There may be more than 2 digits, only pick the first and last
    // Ignore other characters
    // If there is only 1 digit found, repeat it
    let filename: PathBuf = Path::new("data").join("day1-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = extract_total_calibration_values(input_string);
    println!("Calibration value total: v1: {}", value);
}

pub fn day01_puzzle02() {
    // Input as string with multiple lines
    // Each line may contain digits, combine the first and last digit to form a single
    // two-digit number
    // There may be more than 2 digits, only pick the first and last
    // Digits can be spelled out too, that's crazy!
    // Reminds me of my old certification exam where I failed converting spelled out numbers into
    // digits
    // Ignore other characters
    // If there is only 1 digit found, repeat it
    let filename: PathBuf = Path::new("data").join("day1-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = extract_total_calibration_values_v2(input_string);

    println!("Calibration value total: v2: {}", value);
}

fn extract_total_calibration_values(input: String) -> u32 {
    let mut total: u32 = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    // Iterate each lines
    for line in lines.iter() {
        // Collect digits for each line
        let digit = extract_line_value(line.to_string());
        total += digit;
    }
    return total;
}

fn extract_line_value(line: String) -> u32 {
    let mut digits: Vec<u32> = Vec::new();
    for char in line.chars() {
        if let Some(digit) = char.to_digit(10) {
            digits.push(digit);
        }
    }

    if digits.len() >= 2 {
        let tens = digits[0] * 10;
        let ones = digits[digits.len() - 1];
        return tens + ones;
    } else if digits.len() == 1 {
        // Simply repeat the digit
        return (digits[0] * 10) + digits[0];
    } else {
        return 0;
    }
}

fn extract_total_calibration_values_v2(input: String) -> u32 {
    let mut total: u32 = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    // Iterate each lines
    for line in lines.iter() {
        // Collect digits for each line
        let digit = extract_line_value_v2(line.to_string());
        total += digit;
    }
    return total;
}

fn extract_line_value_v2(line: String) -> u32 {
    // Extract digits, but digits can be spelled out 0-9 digits
    // To to this, we need to walk from start to end
    // We will use a start and end pointer
    // The end pointer moves one character at a time
    // Detect a digit or a spelled digit
    // If a digit is detected, collect the digit and move the start pointer in front of the digit
    // If a spelled digit is detected, collect the digit and move the start pointer
    // in front of first char, excluding it, in case there are overlapping words
    let mut digits: Vec<u32> = Vec::new();
    let mut start: usize = 0;
    let mut end: usize;

    for (i, char) in line.chars().enumerate() {
        end = i;
        if let Some(digit) = char.to_digit(10) {
            digits.push(digit);
            // Found a digit, move start to current index
            start = i + 1;
        } else {
            // Try if start to end contains any of the spelled digits
            if end > start && end < line.len() {
                let word = &line[start..=end];
                if let Some(digit_info) = find_spelled_digit(word) {
                    digits.push(digit_info.0);
                    // Find a spelled digit, move start to the last index of that word
                    start = start + digit_info.1 + 1;
                }
            }
        }
    }

    if digits.len() >= 2 {
        let tens = digits[0] * 10;
        let ones = digits[digits.len() - 1];
        return tens + ones;
    } else if digits.len() == 1 {
        // Simply repeat the digit
        return (digits[0] * 10) + digits[0];
    } else {
        return 0;
    }
}

// Return the digit value and the index of the first char of that spelled digit
fn find_spelled_digit(line: &str) -> Option<(u32, usize)> {
    let mut result: Option<(u32, usize)> = None;
    for (i, word) in SPELLED_DIGITS.iter().enumerate() {
        if let Some(index) = line.find(*word) {
            let value: u32 = i as u32 + 1;
            result = Some((value, index));
            break;
        }

    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_puzzle01() {
        let lines = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let total = extract_total_calibration_values(lines.to_string());
        assert_eq!(total, 142);
    }

    #[test]
    fn test_day1_puzzle02() {
        let lines = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let total = extract_total_calibration_values_v2(lines.to_string());
        assert_eq!(total, 281);

        // Test another
        let lines2 = "eightfivesssxxmgthreethreeone1sevenhnz";
        let total2 = extract_total_calibration_values_v2(lines2.to_string());
        assert_eq!(total2, 87);

        // Overlapping words 
        let lines3 = "eighthree";
        let total3 = extract_total_calibration_values_v2(lines3.to_string());
        assert_eq!(total3, 83);

        let lines4 = "sevenine";
        let total4 = extract_total_calibration_values_v2(lines4.to_string());
        assert_eq!(total4, 79);

    }
}
