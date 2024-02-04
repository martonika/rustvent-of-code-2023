use std::{collections::HashMap, fs};

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut calibration = 0;
    // str to num map - use this to parse the text
    let num_map = HashMap::from([
        ('0', 0u32),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
    ]);
    for line in data.lines() {
        // Pass 1 - left to right
        for c in line.chars() {
            if c.is_ascii_digit() {
                calibration += 10 * num_map[&c];
                break;
            }
        }
        // Pass 2 - right to left
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                calibration += num_map[&c];
                break;
            }
        }
    }

    calibration
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut calibration = 0;
    let num_map = HashMap::from([
        ("zero", "z0o"),
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);
    let val_map = HashMap::from([
        ('0', 0u32),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
    ]);

    for line in data.lines() {
        // can't blindly overwrite first match: numbers can overlap, like 'eightwo'
        // here 8 is the wanted num, but iterationg over the map may overwrite two with 2
        // and 'eigh2' will not match anymore -> replacing an inner character with a number is OK
        // e.g. 'eightwo' -> 'eight2o' -> 'e8t2o'
        let mut line_mod = line.to_string();
        for (from, to) in &num_map {
            line_mod = line_mod.replace(from, to);
        }
        // Then pt1:
        // Pass 1 - left to right
        for c in line_mod.chars() {
            if c.is_ascii_digit() {
                calibration += 10 * val_map[&c];
                break;
            }
        }
        // Pass 2 - right to left
        for c in line_mod.chars().rev() {
            if c.is_ascii_digit() {
                calibration += val_map[&c];
                break;
            }
        }
    }

    calibration
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day1/test1.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day1/test2.txt");
        assert_eq!(result, 281);
    }
}
