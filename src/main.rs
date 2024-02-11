pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

fn main() {
    let day1_1 = day1::solve_1("src/day1/input.txt");
    let day1_2 = day1::solve_2("src/day1/input.txt");
    println!("Day 1/1: {day1_1}");
    println!("Day 1/2: {day1_2}");
    println!();

    let day2_1 = day2::solve_1("src/day2/input.txt");
    let day2_2 = day2::solve_2("src/day2/input.txt");
    println!("Day 2/1: {day2_1}");
    println!("Day 2/2: {day2_2}");
    println!();

    let day3_1 = day3::solve_1("src/day3/input.txt");
    let day3_2 = day3::solve_2("src/day3/input.txt");
    println!("Day 3/1: {day3_1}");
    println!("Day 3/2: {day3_2}");
    println!();

    let day4_1 = day4::solve_1("src/day4/input.txt");
    let day4_2 = day4::solve_2("src/day4/input.txt");
    println!("Day 4/1: {day4_1}");
    println!("Day 4/2: {day4_2}");
    println!();

    let day5_1 = day5::solve_1("src/day5/input.txt");
    let day5_2 = day5::solve_2("src/day5/input.txt");
    println!("Day 5/1: {day5_1}");
    println!("Day 5/2: {day5_2}");
    println!();

    let day6_1 = day6::solve_1("src/day6/input.txt");
    let day6_2 = day6::solve_2("src/day6/input.txt");
    println!("Day 6/1: {day6_1}");
    println!("Day 6/2: {day6_2}");
    println!();

    let day7_1 = day7::solve_1("src/day7/input.txt");
    let day7_2 = day7::solve_2("src/day7/input.txt");
    println!("Day 7/1: {day7_1}");
    println!("Day 7/2: {day7_2}");
    println!();

    let day8_1 = day8::solve_1("src/day8/input.txt");
    let day8_2 = day8::solve_2("src/day8/input.txt");
    println!("Day 8/1: {day8_1}");
    println!("Day 8/2: {day8_2}");
    println!();
}
