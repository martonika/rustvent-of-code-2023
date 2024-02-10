use std::fs;

pub fn solve_1(input: &str) -> u64 {
    let binding = fs::read_to_string(input)
        .expect("Can't open file")
        .replace("\r\n", "\n");
    let data = binding.as_str();
    let mut iter = data.split("\n\n");
    let mut seeds: Vec<u64> = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    for group in iter {
        // By iterating over each mapping group, the final updated seed value will be
        // the required location
        let map: Vec<(u64, u64, u64)> = group
            .lines()
            .skip(1) // Skip the "a-to-b map:" lines
            .map(|line| {
                // line here is one line of mappings
                let mapping: Vec<u64> = line
                    .split_ascii_whitespace()
                    .map(|d| d.parse::<u64>().unwrap())
                    .collect();
                (mapping[0], mapping[1], mapping[2])
            })
            .collect();

        seeds = seeds
            .into_iter()
            .map(|s| {
                // Check if the current seed is in range
                if let Some((dst, src, _)) = map.iter().find(|m| s >= m.1 && s < (m.1 + m.2)) {
                    // s is over 'source' and under 'source + length'
                    // e.g. s == 55, dst == 90, src == 50, len > 5
                    // 50 -> 90, 51 -> 91... 55->95
                    // 95 = 55 - 50 + 90
                    s - src + dst
                } else {
                    // s stays the same
                    s
                }
            })
            .collect();
    }
    seeds.into_iter().min().unwrap()
}

pub fn solve_2(input: &str) -> u64 {
    let binding = fs::read_to_string(input)
        .expect("Can't open file")
        .replace("\r\n", "\n");
    let data = binding.as_str();
    let mut iter = data.split("\n\n");
    let seeds_vec: Vec<i64> = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut seeds: Vec<(i64, i64)> = seeds_vec.chunks(2).map(|f| (f[0], f[1])).collect();
    for group in iter {
        // By iterating over each mapping group, the final updated seed value will be
        // the required location
        let mut map: Vec<(i64, i64, i64)> = group
            .lines()
            .skip(1) // Skip the "a-to-b map:" lines
            .map(|line| {
                // line here is one line of mappings
                let mapping: Vec<i64> = line
                    .split_ascii_whitespace()
                    .map(|d| d.parse::<i64>().unwrap())
                    .collect();
                (mapping[0], mapping[1], mapping[2])
            })
            .collect();
        // Sort the mapping so the lowest source is first
        map.sort_by_key(|m| m.1);
        seeds = seeds
            .into_iter()
            .flat_map(|(s_start, s_len)| {
                // Calculate the range
                let mut seed_start = s_start;
                let mut seed_len = s_len;
                let mut seed_end = s_start + s_len - 1; // Inclusive start and end
                let mut new_values = Vec::new();
                for ((map_dest, map_start, map_len), is_last_map) in
                    map.iter().enumerate().map(|(i, m)| (m, i == map.len() - 1))
                {
                    let map_end = map_start + map_len - 1;
                    let map_distance = map_dest - map_start;
                    if seed_end < *map_start || seed_start > map_end {
                        // no overlap with current map
                        // check next map, if any
                        continue;
                    }
                    if seed_start >= *map_start && seed_end <= map_end {
                        // perfect map
                        new_values.push((seed_start + map_distance, seed_len));
                        // don't need to check other maps
                        break;
                    }
                    // First, check if the seed starts before the map
                    // In this case a new range will be added,
                    // from seed_start to map_start-1
                    // e.g. seed: 50-100 (50,51), map: 60-80 (60,21)
                    // need 50-59 + 60-80 (mapped) + 81-100
                    //     (50,10)  (xx,21)          (81,20)
                    // Part 1: (50,10) + (60,41)  (to be mapped later)
                    // Part 2: (xx,21) + 81,20
                    if seed_start < *map_start {
                        // Create the first part, e.g. 50-59 above
                        new_values.push((seed_start, map_start - seed_start));
                        seed_start = *map_start;
                        seed_len -= map_start - seed_start;
                        seed_end = seed_start + seed_len - 1;
                    }
                    // Check (60,41) against (60,21)
                    if seed_end <= map_end {
                        // Perfect map remaining
                        new_values.push((seed_start + map_distance, seed_len));
                        break;
                    }
                    // At this point the seed is longer than the map's range,
                    // OR map 10-40 (10,31), seed 30-50 (30,21)
                    // so split it: (xx,21) + (81,20)
                    // (xx,11) + (41,10)
                    new_values.push((seed_start + map_distance, map_end - seed_start + 1));
                    // Remaining bit - check at the next map
                    seed_start = map_end + 1;
                    // Seed end stays the same
                    seed_len = seed_end - seed_start + 1;
                    if is_last_map {
                        // Last map checked, push the remaining bit
                        new_values.push((seed_start, seed_len));
                    }
                }
                if new_values.len() == 0 {
                    // No overlap with any map
                    // Keep original
                    new_values.push((seed_start, seed_len));
                }
                new_values
            })
            .collect();
        // Delete any duplicates
        seeds.sort_by_key(|k| k.0);
        seeds.dedup();
        //seeds = seeds.into_iter().filter(|f| f.1 != 0).collect();
    }
    seeds.into_iter().min_by_key(|s| s.0).unwrap().0 as u64
    //seeds.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day5/test1.txt");
        assert_eq!(result, 35);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day5/test2.txt");
        assert_eq!(result, 46);
    }
}
