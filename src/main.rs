use advent_of_code_2024::*;
use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse().unwrap();
    let part = args[2].parse().unwrap();

    let now = Instant::now();
    
    let answer = match day {
        1 =>
        {
            match part {
                1 => day1::part1(),
                2 => day1::part2(),
                _ => {
                    println!("Invalid part");
                    0.into()
                }
            }
        }
        _ => {
            println!("Invalid day");
            0.into()
        }
    };
    let elapsed = now.elapsed().as_micros();

    println!(" Day {} Part {} Answer: {:?}\n Time Taken: {} us", day, part, answer, elapsed);
}
