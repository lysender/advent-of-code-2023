
#[derive(Debug, Clone)]
pub struct Card {
    pub matches: u32,
}

pub fn part1(input: &str) -> u32 {
    compute_winning_points(input)
}

pub fn part2(input: &str) -> u32 {
    compute_total_cards(input)
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
    let mut card_copies: Vec<u32> = Vec::new();

    // Collect original cards first
    for line in input.lines() {
        let card_info: Vec<&str> = line.split(":").collect();
        let numbers: Vec<&str> = card_info[1].split(" |").collect();
        let winners = collect_card_numbers(numbers[0]);
        let numbers = collect_card_numbers(numbers[1]);

        let card = Card {
            matches: get_matching_number_count(&winners, &numbers),
        };

        initial_cards.push(card);
        card_copies.push(1);
    }

    // Expand the cards
    for (i, card) in initial_cards.iter().enumerate() {
        let copies = card_copies[i];
        for _ in 0..copies {
            // Add more card copies below the layer
            // Create copies of cards down based on matches
            if card.matches > 0 {
                for i_match in 0..card.matches {
                    let copy_index = i + i_match as usize + 1;
                    // Ensure we don't get pass the bottom of the card list
                    if initial_cards.get(copy_index).is_some() {
                        card_copies[copy_index] += 1;
                    }
                }
            }
        }
    }

    // Sum all cards
    let total_cards: u32 = card_copies.iter().sum();
    total_cards
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
