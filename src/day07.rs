use itertools::Itertools;
use std::{collections::BTreeMap, cmp::Ordering};
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
    Parser,
};

#[derive(Clone, Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone)]
struct Hand<'a> {
    cards: &'a str,
    hand_type: HandType,
    bid: u32,
}

pub fn part1(input: &str) -> u32 {
    if let Ok((_, items)) = parse_input(input) {
        let card_map: BTreeMap<&str, u32> = create_card_types();
        let hands: Vec<Hand> = convert_to_hands(items);
        let sorted_hands = sort_hands(hands, &card_map);
        let winnings: u32 = sorted_hands.iter().enumerate().map(|(i, hand)| {
            (i as u32 + 1) * hand.bid
        }).sum();
        return winnings;
    }
    0
}

pub fn part2(input: &str) -> u32 {
    if let Ok((_, items)) = parse_input(input) {
        let card_map: BTreeMap<&str, u32> = create_card_types_v2();
        let hands: Vec<Hand> = convert_to_hands_v2(items);
        let sorted_hands = sort_hands(hands, &card_map);
        let winnings: u32 = sorted_hands.iter().enumerate().map(|(i, hand)| {
            (i as u32 + 1) * hand.bid
        }).sum();
        return winnings;
    }
    0
}

fn convert_to_hands(items: Vec<(&str, u32)>) -> Vec<Hand> {
    let hands: Vec<Hand> = items.iter().map(|x| {
        Hand {
            cards: x.0,
            hand_type: get_hand_type(x.0),
            bid: x.1,
        }
    }).collect();

    hands
}

fn convert_to_hands_v2(items: Vec<(&str, u32)>) -> Vec<Hand> {
    let hands: Vec<Hand> = items.iter().map(|x| {
        let v_cards = morph_cards(x.0);
        Hand {
            cards: x.0,
            hand_type: get_hand_type(v_cards.as_str()),
            bid: x.1,
        }
    }).collect();

    hands
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, u32)>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(line: &str) -> IResult<&str, (&str, u32)> {
    separated_pair(complete::alphanumeric1, space1, complete::u32).parse(line)
}

fn create_card_types() -> BTreeMap<&'static str, u32> {
    BTreeMap::from([
        ("2", 1),
        ("3", 2),
        ("4", 3),
        ("5", 4),
        ("6", 5),
        ("7", 6),
        ("8", 7),
        ("9", 8),
        ("T", 9),
        ("J", 10),
        ("Q", 11),
        ("K", 12),
        ("A", 13),
    ])
}

fn create_card_types_v2() -> BTreeMap<&'static str, u32> {
    BTreeMap::from([
        ("J", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("T", 10),
        ("Q", 11),
        ("K", 12),
        ("A", 13),
    ])
}

fn sort_hands<'a>(hands: Vec<Hand<'a>>, card_map: &'a BTreeMap<&str, u32>) -> Vec<Hand<'a>> {
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| {
        let a_rank = get_hand_type_rank(&a.hand_type);
        let b_rank = get_hand_type_rank(&b.hand_type);

        if a_rank < b_rank {
            return Ordering::Less;
        } else if a_rank > b_rank {
            return Ordering::Greater;
        } else {
            return cmp_hands_by_chars(a.cards, b.cards, card_map);
        }
    });

    sorted_hands
}

fn cmp_hands_by_chars(a: &str, b: &str, card_map: &BTreeMap<&str, u32>) -> Ordering {
    // Keep on comparing each card until something is not equal
    // Otherwise, return equal at the end
    for (i, a_ch) in a.chars().enumerate() {
        let b_ch = &b[i..i+1];
        let a_str = a_ch.to_string();
        let b_str = b_ch.to_string();
        let a_rank = card_map.get(&a_str.as_str()).unwrap();
        let b_rank = card_map.get(&b_str.as_str()).unwrap();
        let ord = a_rank.cmp(b_rank);
        if ord != Ordering::Equal {
            return ord;
        }
    }
    Ordering::Equal
}

fn get_hand_type(cards: &str) -> HandType {
    // Collect cards to calculate the hand type
    let map = cards.chars().counts();
    let map_len = map.len();
    let values: Vec<&usize> = map.values().sorted().collect();

    if map_len == 1 {
        return HandType::FiveOfAKind;
    } else if map_len == 2 {
        // Two possibilities: four of a kind or full house 
        if *values[1] == 4 {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    } else if map_len == 3 {
        // Two possibilities: three of a kind or two pair 
        if *values[2] == 3 {
            return HandType::ThreeOfAKind;
        } else {
            return HandType::TwoPair;
        }
    } else if map_len == 4 {
        return HandType::OnePair;
    } else {
        return HandType::HighCard;
    }
}

fn get_hand_type_rank(hand_type: &HandType) -> u8 {
    match hand_type {
        HandType::FiveOfAKind => 7,
        HandType::FourOfAKind => 6,
        HandType::FullHouse => 5,
        HandType::ThreeOfAKind => 4,
        HandType::TwoPair => 3,
        HandType::OnePair => 2,
        HandType::HighCard => 1,
    }
}

fn morph_cards(cards: &str) -> String {
    // Collect all similar characters
    let map = cards.chars().counts();

    if let Some(_) = map.get(&'J') {
        // Find the largest char that is not J
        let mut largest_ch: Option<char> = None;
        let mut largest: usize = 0;
        for (i, v) in map.iter() {
            if i != &'J' {
                if v > &largest {
                    largest = *v;
                    largest_ch = Some(*i);
                }
            }
        }

        if let Some(ch) = largest_ch {
            // Replace char with J
            return cards.replace("J", ch.to_string().as_str())
        }
    }

    // Return back the original cards
    cards.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_types() {
        assert_eq!(get_hand_type("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AA8AA"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("23332"), HandType::FullHouse);
        assert_eq!(get_hand_type("TTT98"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("23432"), HandType::TwoPair);
        assert_eq!(get_hand_type("A23A4"), HandType::OnePair);
        assert_eq!(get_hand_type("23456"), HandType::HighCard);
    }

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        // Test final output
        let result = part1(input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_card_morph() {
        assert_eq!(morph_cards("QJJQ2"), "QQQQ2".to_string());
        assert_eq!(morph_cards("32T3K"), "32T3K".to_string());
        assert_eq!(morph_cards("T55J5"), "T5555".to_string());
        assert_eq!(morph_cards("KTJJT"), "KTTTT".to_string());
        assert_eq!(morph_cards("QQQJA"), "QQQQA".to_string());
    }

    #[test]
    fn test_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part2(input);
        assert_eq!(result, 5905);
    }
}
