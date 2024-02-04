use std::fs;

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    draws: Vec<Draw>,
    max_draw: Draw,
}

impl Game {
    pub fn new(id: u32, data: &str) -> Self {
        let mut draws = Vec::new();
        let mut max_draw = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        // each game:
        // "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
        let draws_str: Vec<&str> = data.split("; ").collect();
        for draw in draws_str {
            // each draw:
            // "8 green, 6 blue, 20 red"
            let subsets: Vec<&str> = draw.split(", ").collect();
            let (mut r, mut g, mut b) = (0, 0, 0);

            for sub in subsets {
                // "8 green"
                let (cnt, color) = sub.split_once(' ').unwrap();
                match color {
                    "red" => {
                        r = cnt.parse::<u32>().unwrap();
                        if max_draw.red < r {
                            max_draw.red = r;
                        }
                    }
                    "green" => {
                        g = cnt.parse::<u32>().unwrap();
                        if max_draw.green < g {
                            max_draw.green = g;
                        }
                    }
                    "blue" => {
                        b = cnt.parse::<u32>().unwrap();
                        if max_draw.blue < b {
                            max_draw.blue = b;
                        }
                    }
                    &_ => unreachable!(),
                }
            }
            draws.push(Draw {
                red: r,
                green: g,
                blue: b,
            });
        }
        Self {
            id: id,
            draws: draws,
            max_draw: max_draw,
        }
    }

    pub fn is_possible(&self) -> bool {
        for draw in &self.draws {
            if draw.red > MAX_RED || draw.green > MAX_GREEN || draw.blue > MAX_BLUE {
                return false;
            }
        }
        true
    }

    pub fn game_power(&self) -> u32 {
        self.max_draw.red * self.max_draw.green * self.max_draw.blue
    }
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut possible = 0;
    let mut games: Vec<Game> = Vec::new();
    for line in data.lines() {
        // Line format:
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        let curr_game: Vec<&str> = line.split(": ").collect(); // "Game 3", <the rest>
        let id: u32 = curr_game[0]
            .strip_prefix("Game ")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        games.push(Game::new(id, curr_game[1]));
    }
    for game in games {
        if game.is_possible() {
            possible += game.id;
        }
    }

    possible
}
pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut games: Vec<Game> = Vec::new();
    let mut set_power = 0;
    for line in data.lines() {
        // Line format:
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        let curr_game: Vec<&str> = line.split(": ").collect(); // "Game 3", <the rest>
        let id: u32 = curr_game[0]
            .strip_prefix("Game ")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        games.push(Game::new(id, curr_game[1]));
    }
    for game in games {
        set_power += game.game_power();
    }
    set_power
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day2/test1.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day2/test2.txt");
        assert_eq!(result, 2286);
    }
}
