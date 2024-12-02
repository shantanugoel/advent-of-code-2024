use advent_of_code_2024::*;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse().unwrap();
    let part = args[2].parse().unwrap();

    let now = Instant::now();

    let answer = match (day, part) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        _ => {
            println!("Invalid day or part");
            0.into()
        }
    };
    let elapsed = now.elapsed().as_micros();

    println!(
        " Day {} Part {} Answer: {:?}\n Time Taken: {} us",
        day, part, answer, elapsed
    );
}
