use regex::Regex;
use std::fs;

struct Number {
    value: u32,
    coords: Vec<Coord>,
}

struct Symbol {
    neighbors: Vec<Coord>,
}

impl Number {
    pub fn new(val_str: &str, row: usize, range: Vec<usize>) -> Self {
        let mut coords = Vec::new();
        for col in range {
            coords.push(Coord::new(row, col));
        }
        Self {
            value: (val_str.parse::<u32>().unwrap()),
            coords: (coords),
        }
    }
}

impl Symbol {
    pub fn new(row: usize, col: usize) -> Self {
        let mut coords = Vec::new();
        // Neighbors:
        // 1 (2) (3) (4) 5 6
        // 1 (2)  $  (4) 5 6
        // 1 (2) (3) (4) 5 6
        // (row-1 col-1) (row-1 col) (row-1 col+1)
        //  (row col-1)    symbol     (row col+1)
        // (row+1 col-1) (row+1 col) (row+1 col+1)
        for r in row as i64 - 1..=row as i64 + 1 {
            if r < 0 {
                // Discard -1 as index, r > number of rows is don't care
                continue;
            };
            for c in col as i64 - 1..=col as i64 + 1 {
                if c < 0 {
                    // Discard -1, c > number of columns is don't care
                    continue;
                };
                if r == row as i64 && c == col as i64 {
                    // Discard own cooridnates
                    continue;
                };
                coords.push(Coord::new(r as usize, c as usize));
            }
        }

        Self {
            neighbors: (coords),
        }
    }
}

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}
impl Eq for Coord {}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut part_nums: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut row = 1;
    let re_num = Regex::new(r"[0-9]+").unwrap();
    let re_sym = Regex::new(r"[^0-9.]").unwrap();
    let mut part_num_sum = 0;

    for line in data.lines() {
        let matches: Vec<_> = re_num.find_iter(line).collect();
        // Number
        for m in matches {
            part_nums.push(Number::new(m.as_str(), row, m.range().collect()));
        }
        // Symbol
        let matches: Vec<_> = re_sym.find_iter(line).collect();
        for m in matches {
            symbols.push(Symbol::new(row, m.start()));
        }
        row += 1;
    }

    for sym in &symbols {
        for num in &part_nums {
            for coord in &num.coords {
                if sym.neighbors.contains(coord) {
                    part_num_sum += num.value;
                    break;
                }
            }
        }
    }
    part_num_sum
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut part_nums: Vec<Number> = Vec::new();
    let mut possible_gears: Vec<Symbol> = Vec::new();
    let mut row = 1;
    let re_num = Regex::new(r"[0-9]+").unwrap();
    let re_gear = Regex::new(r"[\*]").unwrap();
    let mut gear_ratio_sum = 0;

    for line in data.lines() {
        let matches: Vec<_> = re_num.find_iter(line).collect();
        // Number
        for m in matches {
            part_nums.push(Number::new(m.as_str(), row, m.range().collect()));
        }
        // Gear
        let matches: Vec<_> = re_gear.find_iter(line).collect();
        for m in matches {
            possible_gears.push(Symbol::new(row, m.start()));
        }
        row += 1;
    }

    for gear in &possible_gears {
        let mut neighbor_parts = Vec::new();
        for num in &part_nums {
            for coord in &num.coords {
                if gear.neighbors.contains(coord) {
                    neighbor_parts.push(num.value);
                    break; // Don't find the same number more than once
                }
            }
        }
        if neighbor_parts.len() == 2 {
            gear_ratio_sum += neighbor_parts.iter().product::<u32>();
        }
    }
    gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day3/test1.txt");
        assert_eq!(result, 4361);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day3/test2.txt");
        assert_eq!(result, 467835);
    }
}
