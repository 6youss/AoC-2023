use std::{cmp::Ordering, collections::HashMap};

use crate::puzzle_input::PUZZLE_INPUT;
mod puzzle_input;
fn main() {
    let mut hands_map: HashMap<&str, u32> = HashMap::new();
    for line in PUZZLE_INPUT.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let [hand, bid_str] = parts.as_slice() {
            if let Ok(bid) = bid_str.parse::<u32>() {
                hands_map.insert(hand, bid);
            }
        }
    }
    let mut hands: Vec<&str> = hands_map.keys().copied().collect();
    hands.sort_by(|hand1, hand2| compare_hands(&hand1, &hand2));
    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            if let Some(bid) = hands_map.get(hand) {
                (index as u32 + 1) * bid
            } else {
                0
            }
        })
        .sum();
    println!("⭐ {}", sum);
}

fn compare_hands(hand1: &str, hand2: &str) -> Ordering {
    let type_comp = compare_hands_type(hand1, hand2);
    match type_comp {
        Ordering::Equal => {
            return compare_same_type_hands(hand1, hand2);
        }
        _ => return type_comp,
    }
}

fn compare_same_type_hands(hand1: &str, hand2: &str) -> Ordering {
    for (card1, card2) in hand1.chars().zip(hand2.chars()) {
        if card1 != card2 {
            let order1 = get_card_order(&card1).unwrap();
            let order2 = get_card_order(&card2).unwrap();
            if order1 > order2 {
                return Ordering::Greater;
            } else if order1 < order2 {
                return Ordering::Less;
            }
        }
    }
    return Ordering::Equal;
}

fn compare_hands_type(hand1: &str, hand2: &str) -> Ordering {
    let type1 = get_hand_type(&hand1);
    let type2 = get_hand_type(&hand2);

    let order1 = get_hand_order(&type1);
    let order2 = get_hand_order(&type2);

    if order1 > order2 {
        Ordering::Greater
    } else if order1 < order2 {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn get_hand_order(hand: &HandType) -> u32 {
    match hand {
        HandType::FiveOfAKind => 7,
        HandType::FourOfAKind => 6,
        HandType::FullHouse => 5,
        HandType::ThreeOfAKind => 4,
        HandType::TwoPair => 3,
        HandType::OnePair => 2,
        HandType::HighCard => 1,
    }
}
fn get_card_order(card: &char) -> Option<u32> {
    match card {
        'A' => Some(13),
        'K' => Some(12),
        'Q' => Some(11),
        'T' => Some(9),
        '9' => Some(8),
        '8' => Some(7),
        '7' => Some(6),
        '6' => Some(5),
        '5' => Some(4),
        '4' => Some(3),
        '3' => Some(2),
        '2' => Some(1),
        'J' => Some(0),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_type(hand: &str) -> HandType {
    let mut count_map: HashMap<char, u32> = HashMap::new();

    for card in hand.chars() {
        if let Some(count) = count_map.get(&card) {
            count_map.insert(card, count + 1);
        } else {
            count_map.insert(card, 1);
        }
    }

    let mut counts: Vec<u32> = count_map.values().cloned().collect();
    counts.sort();
    counts.reverse();

    // handle the J joker card
    let has_a_j = hand.contains('J');

    if has_a_j {
        let j_count = count_map.get(&'J').unwrap();
        if *j_count < 5 {}
    }

    let counts_len = counts.len();
    if counts_len == 1 {
        HandType::FiveOfAKind
    } else if counts_len == 2 && counts[0] == 4 {
        HandType::FourOfAKind
    } else if counts_len == 2 && counts[0] == 3 {
        HandType::FullHouse
    } else if counts_len == 3 && counts[0] == 3 {
        HandType::ThreeOfAKind
    } else if counts_len == 3 && counts[0] == 2 && counts[1] == 2 {
        HandType::TwoPair
    } else if counts_len == 4 && counts[0] == 2 && counts[1] == 1 {
        HandType::OnePair
    } else if counts_len == 5 {
        HandType::HighCard
    } else {
        HandType::FullHouse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_hand_type() {
        assert_eq!(get_hand_type("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AAAA2"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("A2AAA"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("A22AA"), HandType::FullHouse);
        assert_eq!(get_hand_type("AAAKK"), HandType::FullHouse);
        assert_eq!(get_hand_type("A21AA"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("AAA23"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("AA332"), HandType::TwoPair);
        assert_eq!(get_hand_type("AAK32"), HandType::OnePair);
    }

    #[test]
    fn test_compare_hands_type() {
        assert_eq!(compare_hands_type("AAAAA", "AAAAA"), Ordering::Equal);
        assert_eq!(compare_hands_type("A2AAA", "AAAAA"), Ordering::Less);
        assert_eq!(compare_hands_type("A22AA", "A1ACA"), Ordering::Greater);
        assert_eq!(compare_hands_type("33332", "2AAAA"), Ordering::Equal);
        assert_eq!(compare_hands_type("77888", "77788"), Ordering::Equal);
    }

    #[test]
    fn test_compare_hands() {
        assert_eq!(compare_hands("33332", "2AAAA"), Ordering::Greater);
        assert_eq!(compare_hands("77888", "77788"), Ordering::Greater);
    }
}
