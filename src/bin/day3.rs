
use std::{fs, path::PathBuf};
use std::path::Path;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct PartSymbol {
    position: (usize, usize),
    length: usize,
    symbol: Option<char>,
    number: Option<u32>,
    is_number: bool,
}

fn main() {
    part1();
    part2();
}

pub fn part1() {
    let filename: PathBuf = Path::new("data").join("day3-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = sum_part_numbers(input_string.as_str());
    println!("Total of part numbers: {}", value);
}

fn sum_part_numbers(input: &str) -> u32 {
    // Map coordinates of all special characters, excluding "."
    // Find all adjacent numbers, but do not repeat number if already accounted for
    // Sum all numbers
    let mut symbols: Vec<PartSymbol> = Vec::new();
    let mut rows: Vec<Vec<PartSymbol>> = Vec::new();
    let mut counted: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut total: u32 = 0;

    for (i, line) in input.lines().enumerate() {
        let line_symbols = parse_line_for_symbols(line, i);
        for line_symbol in line_symbols.iter() {
            symbols.push(line_symbol.clone());
        }

        rows.push(line_symbols);
    }

    let markers: Vec<&PartSymbol> = symbols.iter().filter(|x| !x.is_number).collect();

    for marker in markers.iter() {
        // Find all numbers adjacent to the symbol
        let same_line_symbols = &rows[marker.position.0];
        // Find symbols on same line
        for symbol in same_line_symbols.iter() {
            if symbol.is_number {
                // Find symbols to the left
                if (symbol.position.1 + symbol.length) == marker.position.1 {
                    if !counted.contains(&symbol.position) {
                        total += symbol.number.unwrap();
                        counted.insert(symbol.position);
                    }
                }

                // Find symbols to the right
                if (symbol.position.1 > 0) && symbol.position.1 - 1 == marker.position.1 {
                    if !counted.contains(&symbol.position) {
                        total += symbol.number.unwrap();
                        counted.insert(symbol.position);
                    }
                }
            }
        }

        // Find symbols above
        if marker.position.0 > 0 {
            if let Some(above_symbols) = &rows.get(marker.position.0 - 1) {
                for symbol in above_symbols.iter() {
                    if symbol.is_number && symbol.length > 0 {
                        if is_hitbox(marker.position.1, symbol.position.1, symbol.length) {
                            if !counted.contains(&symbol.position) {
                                total += symbol.number.unwrap();
                                counted.insert(symbol.position);
                            }
                        }
                    }
                }
            }
        }

        // Find symbols below
        if let Some(below_symbols) = &rows.get(marker.position.0 + 1) {
            for symbol in below_symbols.iter() {
                if symbol.is_number && symbol.length > 0 {
                    if is_hitbox(marker.position.1, symbol.position.1, symbol.length) {
                        if !counted.contains(&symbol.position) {
                            total += symbol.number.unwrap();
                            counted.insert(symbol.position);
                        }
                    }
                }
            }
        }
    }

    total
}

fn parse_line_for_symbols(line: &str, line_number: usize) -> Vec<PartSymbol> {
    let mut symbols: Vec<PartSymbol> = Vec::new();
    let mut digits_buffer = String::new(); 
    let mut last_index: usize = 0;

    for (i, char) in line.chars().enumerate() {
        last_index = i;
        if char.is_numeric() {
            // Found a digit, push it to buffer in case this is a part of a bigger number
            digits_buffer.push(char);
        } else if char == '.' {
            // Flush digits buffer
            if digits_buffer.len() > 0 {
                if let Some(symbol) = convert_to_part_symbol(digits_buffer.as_str(), line_number, last_index) {
                    symbols.push(symbol);
                }
            }
            digits_buffer = String::new();
        } else {
            if digits_buffer.len() > 0 {
                // Flush digits buffer
                if let Some(symbol) = convert_to_part_symbol(digits_buffer.as_str(), line_number, last_index) {
                    symbols.push(symbol);
                }
            }
            digits_buffer = String::new();

            // New symbol found, add to symbols
            let symbol = PartSymbol {
                position: (line_number, i),
                length: 1,
                number: None,
                symbol: Some(char),
                is_number: false,
            };

            symbols.push(symbol);
        }
    }

    if digits_buffer.len() > 0 {
        if let Some(symbol) = convert_to_part_symbol(digits_buffer.as_str(), line_number, last_index) {
            symbols.push(symbol);
        }
    }
    symbols
}

fn convert_to_part_symbol(digits_buffer: &str, line_number: usize, last_index: usize) -> Option<PartSymbol> {
    if let Ok(number) = digits_buffer.parse::<u32>() {
        let mut y: usize = 0;
        if last_index >= digits_buffer.len() {
            y = last_index - digits_buffer.len();
        }
        let symbol = PartSymbol {
            position: (line_number, y),
            length: digits_buffer.len(),
            number: Some(number),
            symbol: None,
            is_number: true,
        };
        return Some(symbol);
    }

    None
}

fn is_hitbox_edges(col: usize, index: usize, length: usize) -> bool {
    // Start is exact match? 
    if col == index {
        return true;
    }

    // End is at exact position?
    if (index + length - 1) == col {
        return true;
    }

    // End is diagonal left?
    if (index + length) == col {
        return true;
    }

    // Start is diagonal right?
    if index > 0 && (index - 1) == col {
        return true;
    }

    return false;
}

fn is_hitbox_middle(col: usize, index: usize, length: usize) -> bool {
    // Should cover the following:
    // index aligned with col
    // index diagonal to the right
    // end of symbol is aligned with col
    // end of symbol is diagonal to the left
    // col hits the body of the symbol
    let mut start = index;
    if index >= length {
        start -= 1;
    }
    let end = start + length;
    col >= start && col <= end
}

fn is_hitbox(col: usize, index: usize, length: usize) -> bool {
    if is_hitbox_middle(col, index, length) {
        return true;
    }
    return is_hitbox_edges(col, index, length);
}


fn compute_gears(input: &str) -> u32 {
    // Gears are two parts that are adjacent to each other via the * symbol
    // Map coordinates of all special characters, excluding "."
    // Find all adjacent numbers, but do not repeat number if already accounted for
    // Sum all numbers
    let mut symbols: Vec<PartSymbol> = Vec::new();
    let mut rows: Vec<Vec<PartSymbol>> = Vec::new();
    let mut total: u32 = 0;

    for (i, line) in input.lines().enumerate() {
        let line_symbols = parse_line_for_symbols(line, i);
        for line_symbol in line_symbols.iter() {
            symbols.push(line_symbol.clone());
        }

        rows.push(line_symbols);
    }

    let mut gear_symbols: Vec<&PartSymbol> = Vec::new(); 

    for symbol in symbols.iter() {
        if !symbol.is_number && symbol.symbol.unwrap() == '*' {
           gear_symbols.push(symbol);
        }
    }

    for gear in gear_symbols.clone().iter() {
        let gear_ratio = find_gear_ratio(gear, &rows);
        total += gear_ratio;
    }

    total
}

fn find_gear_ratio(gear: &PartSymbol, rows: &Vec<Vec<PartSymbol>>) -> u32 {
    let mut parts: Vec<&PartSymbol> = Vec::new();
    // Find all numbers adjacent to the symbol
    let same_line_symbols = &rows[gear.position.0];
    // Find symbols on same line
    for symbol in same_line_symbols.iter() {
        if symbol.is_number {
            // Find symbols to the left
            if (symbol.position.1 + symbol.length) == gear.position.1 {
                parts.push(symbol);
            }

            // Find symbols to the right
            if (symbol.position.1 > 0) && symbol.position.1 - 1 == gear.position.1 {
                parts.push(symbol);
            }
        }
    }

    // Find symbols above
    if gear.position.0 > 0 {
        if let Some(above_symbols) = &rows.get(gear.position.0 - 1) {
            for symbol in above_symbols.iter() {
                if symbol.is_number && symbol.length > 0 {
                    if is_hitbox(gear.position.1, symbol.position.1, symbol.length) {
                        parts.push(symbol);
                    }
                }
            }
        }
    }

    // Find symbols below
    if let Some(below_symbols) = &rows.get(gear.position.0 + 1) {
        for symbol in below_symbols.iter() {
            if symbol.is_number && symbol.length > 0 {
                if is_hitbox(gear.position.1, symbol.position.1, symbol.length) {
                    parts.push(symbol);
                }
            }
        }
    }

    if parts.len() == 2 {
        return parts[0].number.unwrap() * parts[1].number.unwrap();
    }
    return 0;
}

pub fn part2() {
    let filename: PathBuf = Path::new("data").join("day3-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = compute_gears(input_string.as_str());
    println!("Total gears: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_puzzle01_data1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let total = sum_part_numbers(input);
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_day3_puzzle01_data2() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";
        let total = sum_part_numbers(input);
        assert_eq!(total, 413);
    }

    #[test]
    fn test_day3_puzzle01_data3() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let total = sum_part_numbers(input);
        assert_eq!(total, 925);
    }
    
    #[test]
    fn test_day3_puzzle01_data4() {
        let input = ".......5......
..7*..*.......
...*13*.......
.......15.....";
        let total = sum_part_numbers(input);
        assert_eq!(total, 40);
    }

    #[test]
    fn test_day3_puzzle01_data5() {
        let input = "100
200";
        let total = sum_part_numbers(input);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_day3_puzzle01_data6() {
        let input = "503+";
        let total = sum_part_numbers(input);
        assert_eq!(total, 503);
    }

    #[test]
    fn test_day3_puzzle01_data7() {
        let input = "............
..789.......
...+........
............";
        let total = sum_part_numbers(input);
        assert_eq!(total, 789);
    }

    #[test]
    fn test_day3_puzzle02() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let gears = compute_gears(input);
        assert_eq!(gears, 467835);

    }
}
