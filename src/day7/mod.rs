use std::{collections::HashMap, fs};

struct Hand {
    cards: Vec<u8>,
    hand_type: u8,
    bid: u64,
}

impl Hand {
    fn new(cards: Vec<u8>, hand_type: u8, bid: u64) -> Self {
        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

pub fn solve_1(input: &str) -> u64 {
    let mut hands: Vec<Hand> = Vec::new();
    let data = fs::read_to_string(input).expect("Can't open file");
    for line in data.lines() {
        if let Some((hand, bid_str)) = line.split_once(' ') {
            let cards: Vec<u8> = hand
                .chars()
                .map(|c| match c {
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => unreachable!(),
                })
                .collect();
            let mut type_helper: HashMap<u8, u8> = HashMap::new();
            for card in &cards {
                match type_helper.get(&card) {
                    Some(c) => type_helper.insert(*card, *c + 1),
                    None => type_helper.insert(*card, 1),
                };
            }
            let hand_type = match type_helper.len() {
                1 => 7, //Five of a kind
                2 => type_match_2(&type_helper),
                3 => type_match_3(&type_helper),
                4 => 2, // One pair
                5 => 1, // High card
                _ => unreachable!(),
            };
            let bid = bid_str.parse::<u64>().unwrap();
            hands.push(Hand::new(cards.clone(), hand_type, bid));
        }
    }
    hands.sort_by_key(|k| {
        (
            k.hand_type,
            k.cards[0],
            k.cards[1],
            k.cards[2],
            k.cards[3],
            k.cards[4],
        )
    });
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

pub fn solve_2(input: &str) -> u64 {
    let mut hands: Vec<Hand> = Vec::new();
    let data = fs::read_to_string(input).expect("Can't open file");
    for line in data.lines() {
        if let Some((hand, bid_str)) = line.split_once(' ') {
            let cards: Vec<u8> = hand
                .chars()
                .map(|c| match c {
                    'J' => 1, // Joker is the "worst" card now
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'T' => 10,
                    'Q' => 11,
                    'K' => 12,
                    'A' => 13,
                    _ => unreachable!(),
                })
                .collect();
            let mut type_helper: HashMap<u8, u8> = HashMap::new();
            for card in &cards {
                match type_helper.get(&card) {
                    Some(c) => type_helper.insert(*card, *c + 1),
                    None => type_helper.insert(*card, 1),
                };
            }
            // Remove jokers, if any, and save the number of them
            let jokers = match type_helper.remove(&1) {
                Some(j) => j,
                None => 0,
            };
            // Non-joker match
            let mut hand_type;
            println!("Hand: {hand}");
            if jokers == 0 {
                println!("No J");
                hand_type = match type_helper.len() {
                    1 => 7, // Five of a kind
                    2 => type_match_2(&type_helper),
                    3 => type_match_3(&type_helper),
                    4 => 2, // One pair
                    5 => 1, // High card
                    _ => unreachable!(),
                };
            } else {
                println!("{jokers} J");
                hand_type = match type_helper.len() {
                    // Five of a kind (x of the same + J)
                    0 => 7, // 5 jokers
                    1 => 7,
                    // Four of a kind or full house
                    // J | AAKK -> FH
                    // J | AAAK -> 4K
                    // JJ | AAK -> 4K
                    // JJJ | AK -> 4K
                    // If there's 1 card from any one kind, it's 4 of a kind
                    2 => type_match_2_joker(&type_helper),
                    // Three of a kind
                    // J | AAKQ -> 3K
                    // JJ | AKQ -> 3K
                    3 => 4,
                    // One pair
                    // J | AKQT
                    4 => 2, // One pair
                    // High card not possible
                    _ => unreachable!(),
                };
            }
            let bid = bid_str.parse::<u64>().unwrap();
            hands.push(Hand::new(cards.clone(), hand_type, bid));
        }
    }
    hands.sort_by_key(|k| {
        (
            k.hand_type,
            k.cards[0],
            k.cards[1],
            k.cards[2],
            k.cards[3],
            k.cards[4],
        )
    });
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

fn type_match_2(map: &HashMap<u8, u8>) -> u8 {
    // Map length is 2 - Four of a kind or full house
    for (_, v) in map {
        if *v == 4 || *v == 1 {
            // Four of a kind
            return 6;
        } else {
            // Full house
            return 5;
        }
    }
    return 0;
}

fn type_match_3(map: &HashMap<u8, u8>) -> u8 {
    // Map length is 3 - Three of a kind or two pairs
    for (_, v) in map {
        if *v == 3 {
            // Three of a kind
            return 4;
        }
        if *v == 2 {
            // Two pairs
            return 3;
        }
    }
    return 0;
}

fn type_match_2_joker(map: &HashMap<u8, u8>) -> u8 {
    for (_, v) in map {
        if *v == 1 {
            // Four of a kind
            return 6;
        } else {
            // Can't decide
        }
    }
    return 5;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day7/test1.txt");
        assert_eq!(result, 6440);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day7/test2.txt");
        assert_eq!(result, 5905);
    }
}
