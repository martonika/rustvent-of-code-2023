use std::{collections::HashSet, fs};

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut points = 0;
    for line in data.lines() {
        let numbers = line.split_once(": ").unwrap().1;
        let (own, win) = numbers.split_once(" | ").unwrap();
        let own_set: HashSet<u32> = own
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let win_set: HashSet<u32> = win
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let win_cnt = win_set.intersection(&own_set).count();
        if win_cnt > 0 {
            points += 2u32.pow(win_cnt as u32 - 1);
        }
    }
    points
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut cards_cnt = vec![0; 211];
    cards_cnt.fill(0);
    for (current_card, line) in data.lines().enumerate() {
        let numbers = line.split_once(": ").unwrap().1;
        let (own, win) = numbers.split_once(" | ").unwrap();
        let own_set: HashSet<u32> = own
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let win_set: HashSet<u32> = win
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let win_cnt = win_set.intersection(&own_set).count();
        // Count the current card
        cards_cnt[current_card] += 1;
        for i in 1..=win_cnt {
            cards_cnt[current_card + i] += cards_cnt[current_card];
        }
    }
    let copies = cards_cnt.iter().sum();
    copies
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day4/test1.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day4/test2.txt");
        assert_eq!(result, 30);
    }
}
