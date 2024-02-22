use std::fs;

pub fn solve_1(input: &str) -> i64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut prediction_sum = 0;
    for line in data.lines() {
        let sensor_data: Vec<_> = line
            .split_ascii_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect();
        prediction_sum += diff(&sensor_data);
    }
    prediction_sum
}

pub fn solve_2(input: &str) -> i64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut prediction_sum = 0;
    for line in data.lines() {
        let sensor_data: Vec<_> = line
            .split_ascii_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect();
        prediction_sum += diff_first(&sensor_data);
    }
    prediction_sum
}

fn diff(data: &Vec<i64>) -> i64 {
    let mut ret = 0;
    if !data.iter().all(|f| *f == 0) {
        let mut new_data = Vec::new();
        for i in 1..data.len() {
            new_data.push(data[i] - data[i - 1]);
        }
        ret += diff(&new_data);
    }
    ret += data.last().unwrap();
    ret
}

fn diff_first(data: &Vec<i64>) -> i64 {
    let mut ret = 0;
    if !data.iter().all(|f| *f == 0) {
        let mut new_data = Vec::new();
        for i in 1..data.len() {
            new_data.push(data[i] - data[i - 1]);
        }
        ret = diff_first(&new_data) - ret;
    }
    ret = data.first().unwrap() - ret;
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day9/test1.txt");
        assert_eq!(result, 114);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day9/test2.txt");
        assert_eq!(result, 2);
    }
}
