
use crate::utils::{self, Answer};

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let lines = utils::read_lines("./inputs/day1");
    lines
        .into_iter()
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

pub fn part1() -> Answer {
    0.into()
}

pub fn part2() -> Answer {
    0.into()
}