
use std::{fs, path::PathBuf};
use std::path::Path;

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    matches: u32,
}

pub fn day04_puzzle01() {
    let filename: PathBuf = Path::new("data").join("day4-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = compute_winning_points(input_string.as_str());
    println!("Winning points: {}", value);
}

pub fn day04_puzzle02() {
    let filename: PathBuf = Path::new("data").join("day4-input.txt");
    let input_string = fs::read_to_string(filename).unwrap();
    let value = compute_total_cards(input_string.as_str());
    println!("Total cards: {}", value);
}

fn compute_winning_points(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let card_info: Vec<&str> = line.split(":").collect();
        let numbers: Vec<&str> = card_info[1].split(" |").collect();
        let winners = collect_card_numbers(numbers[0]);
        let numbers = collect_card_numbers(numbers[1]);

        // Find matches
        let matches: u32 = get_matching_number_count(&winners, &numbers);
        let points = compute_card_points(matches);
        total += points;
    }
    total
}

fn get_matching_number_count(winners: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    let matches: u32 = numbers.iter().map(|n| {
        match winners.contains(n) {
            true => 1,
            false => 0,
        }
    }).sum();
    matches
}

fn get_card_id(line: &str) -> u32 {
    let str = line.replace("Card", "").trim().to_string();
    let id: u32 = str.parse::<u32>().unwrap();
    id
}

fn collect_card_numbers(line: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut start: usize = 0;
    while start < line.len() {
        let num_str = &line[start..start + 3].trim().to_string();
        let num: u32 = num_str.parse::<u32>().unwrap();
        numbers.push(num);

        // Next 3 chars
        start += 3;
    }
    numbers
}

fn compute_card_points(matches: u32) -> u32 {
    let mut value: u32 = 0;
    if matches > 0 {
        value = 1;
        for _ in 0..matches - 1 {
            value *= 2;
        }
    }

    value
}

fn compute_total_cards(input: &str) -> u32 {
    let mut initial_cards: Vec<Card> = Vec::new();
    let mut expanding_cards: Vec<Vec<Card>> = Vec::new();

    // Collect original cards first
    for line in input.lines() {
        let card_info: Vec<&str> = line.split(":").collect();
        let id = get_card_id(card_info[0]);
        let numbers: Vec<&str> = card_info[1].split(" |").collect();
        let winners = collect_card_numbers(numbers[0]);
        let numbers = collect_card_numbers(numbers[1]);

        let card = Card {
            id,
            matches: get_matching_number_count(&winners, &numbers),
        };

        let mut row: Vec<Card> = Vec::new();
        row.push(card.clone());
        expanding_cards.push(row);

        initial_cards.push(card);
    }

    for layer in 0..expanding_cards.len() {
        // Clone the row so we can mutate rows below it
        let row = expanding_cards[layer].clone();
        for row_card in row.iter() {
            if row_card.matches > 0 {
                // Add more cards below the layer
                // Create copies of cards down based on matches
                for i_match in 0..row_card.matches {
                    let copy_index = layer + i_match as usize + 1;
                    if let Some(additional_card) = initial_cards.get(copy_index) {
                        expanding_cards[copy_index].push(Card {
                            id: additional_card.id,
                            matches: additional_card.matches,
                        });
                    }
                }
            }
        }
    }

    // Sum all cards
    let total_cards: usize  = expanding_cards.iter().map(|row| {
        row.len()
    }).sum();

    total_cards as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_puzzle01_data1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let total = compute_winning_points(input);
        assert_eq!(total, 13);
    }

    #[test]
    fn test_day4_puzzle01_data2() {
        let input = "Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card  2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card  3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card  4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card  5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 16: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let total = compute_winning_points(input);
        assert_eq!(total, 13);
    }

    #[test]
    fn test_card_points() {
        assert_eq!(compute_card_points(4), 8);
        assert_eq!(compute_card_points(2), 2);
        assert_eq!(compute_card_points(1), 1);
        assert_eq!(compute_card_points(0), 0);
    }

    #[test]
    fn test_day4_puzzle02_data1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let total = compute_total_cards(input);
        assert_eq!(total, 30);
    }
}
