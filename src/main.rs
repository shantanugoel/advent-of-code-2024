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
        (3, 1) => day3::part1(),
        (3, 2) => day3::part2(),
        (3, 3) => day3::visualize_part2(),
        (4, 1) => day4::part1(),
        (4, 2) => day4::part2(),
        (4, 3) => day4::visualize_part2(),
        (5, 1) => day5::part1(),
        (5, 2) => day5::part2(),
        (6, 1) => day6::part1("./inputs/day6"),
        (6, 2) => day6::part2("./inputs/day6"),
        (7, 1) => day7::part1("./inputs/day7"),
        (7, 2) => day7::part2("./inputs/day7"),
        (8, 1) => day8::part1("./inputs/day8"),
        (8, 2) => day8::part2("./inputs/day8"),
        (9, 1) => day9::part1("./inputs/day9"),
        (9, 2) => day9::part2("./inputs/day9"),
        (10, 1) => day10::part1("./inputs/day10"),
        (10, 2) => day10::part2("./inputs/day10"),
        (11, 1) => day11::parts("./inputs/day11", 25),
        (11, 2) => day11::parts("./inputs/day11", 75),
        (12, 1) => day12::part1("./inputs/day12"),
        (12, 2) => day12::part2("./inputs/day12"),
        (13, 1) => day13::part1("./inputs/day13"),
        (13, 2) => day13::part2("./inputs/day13"),
        (14, 1) => day14::part1("./inputs/day14", 101, 103),
        (14, 2) => day14::part2("./inputs/day14", 101, 103),
        (15, 1) => day15::part1("./inputs/day15"),
        (15, 2) => day15::part2("./inputs/day15"),
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
