use num::integer::lcm;
use std::{collections::HashMap, fs};

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut data_iter = data.lines();
    // True == right, false == left
    let instructions = data_iter.next().unwrap().chars().map(|c| c == 'R').cycle();
    data_iter.next(); // Skip newline
    let node_map: HashMap<&str, (&str, &str)> = data_iter
        .map(|line| {
            let current = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (current, (left, right))
        })
        .collect();
    let mut current = "AAA";
    let target = "ZZZ";

    for (i, dir) in instructions.enumerate() {
        if current == target {
            return i as u32;
        }
        let (l, r) = node_map[current];
        current = if dir == true { r } else { l };
    }
    0
}

pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut data_iter = data.lines();
    // True == right, false == left
    let instructions = data_iter.next().unwrap().chars().map(|c| c == 'R').cycle();
    data_iter.next(); // Skip newline
    let node_map: HashMap<&str, (&str, &str)> = data_iter
        .map(|line| {
            let current = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (current, (left, right))
        })
        .collect();

    let currents: Vec<_> = node_map
        .iter()
        .filter(|n| n.0.ends_with('A'))
        .map(|n| *n.0)
        .collect();

    // All paths loop with different cycles -> lowest common multiplier is sufficient
    let mut path_len = 1;

    for mut curr in currents {
        for (i, dir) in instructions.clone().enumerate() {
            if curr.ends_with('Z') {
                path_len = lcm(path_len, i as u64);
                break;
            }
            let (l, r) = node_map[curr];
            curr = if dir == true { r } else { l };
        }
    }
    path_len
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_1() {
        let result = solve_1("src/day8/test1.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_2() {
        let result = solve_1("src/day8/test2.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_1() {
        let result = solve_2("src/day8/test3.txt");
        assert_eq!(result, 6);
    }
}
