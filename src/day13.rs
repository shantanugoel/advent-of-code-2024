use crate::utils::{self, Answer};
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ButtonMove {
    x: i32,
    y: i32,
}

fn get_input(input: &str) -> Vec<(ButtonMove, ButtonMove, Point)> {
    let mut result: Vec<(ButtonMove, ButtonMove, Point)> = Vec::new();
    let lines = utils::read_lines(input);

    let button_regex = Regex::new(r"\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"=(\d+), Y=(\d+)").unwrap();

    for input_x in lines.chunks(4) {
        let mut captures = button_regex.captures(input_x[0].as_str()).unwrap();
        let a_cfg = ButtonMove {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
        };
        captures = button_regex.captures(input_x[1].as_str()).unwrap();
        let b_cfg = ButtonMove {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
        };
        captures = prize_regex.captures(input_x[2].as_str()).unwrap();
        let prize_cfg = Point {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
        };
        result.push((a_cfg, b_cfg, prize_cfg));
    }

    result
}

pub fn part1(input: &str) -> Answer {
    let config = get_input(input);
    println!("{:?}", config);
    todo!();
}

pub fn part2(input: &str) -> Answer {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day13_sample"), 0u64.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day13_sample"), 0u64.into());
    }
}
