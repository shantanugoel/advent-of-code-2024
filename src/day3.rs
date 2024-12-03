use crate::utils::{self, Answer};
use regex::Regex;

fn get_input() -> Vec<(i64, i64)> {
    let lines = utils::read_lines("./inputs/day3");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut input: Vec<(i64, i64)> = Vec::new();
    for line in lines.iter() {
        for (_, [a, b]) in re.captures_iter(line).map(|c| c.extract()) {
            input.push((a.parse().unwrap(), b.parse().unwrap()));
        }
    }
    input
}

pub fn part1() -> Answer {
    let input = get_input();
    let mut sum: i64 = 0;
    for (a, b) in input.iter() {
        sum += a * b;
    }
    sum.into()
}

pub fn part2() -> Answer {
    0.into()
}
