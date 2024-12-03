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

#[derive(Debug, PartialEq, Eq)]
enum MulState {
    Enabled,
    Disabled,
}

fn get_input_part2() -> Vec<(i64, i64)> {
    let lines = utils::read_lines("./inputs/day3");
    let re = Regex::new(r"(do)\(\)|(don)\'t\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut input: Vec<(i64, i64)> = Vec::new();
    let mut state = MulState::Enabled;
    for line in lines.iter() {
        for capture in re.captures_iter(line) {
            if capture.get(1).is_some() {
                state = MulState::Enabled;
            } else if capture.get(2).is_some() {
                state = MulState::Disabled;
            } else if state == MulState::Enabled {
                let a = capture.get(3).unwrap().as_str();
                let b = capture.get(4).unwrap().as_str();
                input.push((a.parse().unwrap(), b.parse().unwrap()));
            }
        }
    }
    input
}

pub fn part2() -> Answer {
    let input = get_input_part2();
    let mut sum: i64 = 0;
    for (a, b) in input.iter() {
        sum += a * b;
    }
    sum.into()
}
