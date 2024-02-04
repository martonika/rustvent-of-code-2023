pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
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
}