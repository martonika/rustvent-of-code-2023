use std::fs;

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut data_lines = data.lines();
    let times_str = data_lines.next().unwrap();
    let records_str = data_lines.next().unwrap();
    let mut beat_num = 1;
    let times: Vec<u32> = times_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
    let records: Vec<u32> = records_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<u32>().unwrap())
        .collect();

    assert!(times.len() == records.len());
    for (i, t) in times.iter().enumerate() {
        let to_beat = records[i];
        let time = *t;
        let mut min_hold = u32::MAX;
        let mut max_hold = u32::MIN;
        for hold in 1..time {
            let dist = (time - hold) * hold;
            if dist > to_beat {
                if min_hold > max_hold {
                    // Found first
                    min_hold = hold;
                }
                max_hold = hold;
            } else if max_hold > min_hold {
                // Got max_hold
                break;
            }
        }
        beat_num *= max_hold - min_hold + 1;
    }

    beat_num
}

pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut data_lines = data.lines();
    let time_str = data_lines.next().unwrap();
    let record_str = data_lines.next().unwrap();

    let time = time_str
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let record = record_str
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    // The midpoint will be the largest distance
    let mut high = time / 2;
    let mut low = 1u64;
    let mut current_time = (high + low) / 2;
    // Find min w/ binary search
    let min_time = loop {
        // If the current is higher, but the previous is lower than the record, we found the minimum
        if get_dist(current_time, time) > record && get_dist(current_time - 1, time) <= record {
            // found min_dist
            break current_time;
        }
        if get_dist(current_time, time) > record {
            // can be lower
            high = current_time - 1;
            current_time = (high + low) / 2;
        } else if get_dist(current_time, time) <= record {
            // must be higher
            low = current_time + 1;
            current_time = (high + low) / 2;
        }
    };

    // Now find max the same way
    high = time;
    low = time / 2;
    current_time = (high + low) / 2;
    let max_time = loop {
        // If current is higher, but the next is lower, we found the maximum
        if get_dist(current_time, time) > record && get_dist(current_time + 1, time) <= record {
            // found max_dist
            break current_time;
        }
        if get_dist(current_time, time) > record {
            // must be higher
            low = current_time + 1;
            current_time = (high + low) / 2;
        } else if get_dist(current_time, time) <= record {
            // can be lower
            high = current_time - 1;
            current_time = (high + low) / 2;
        }
    };

    max_time - min_time + 1
}

fn get_dist(hold: u64, time: u64) -> u64 {
    (time - hold) * hold
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day6/test1.txt");
        assert_eq!(result, 288);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day6/test2.txt");
        assert_eq!(result, 71503);
    }
}
