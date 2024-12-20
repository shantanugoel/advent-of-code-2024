use crate::utils::{self, Answer};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i128,
    y: i128,
}

impl Point {
    fn press_button(&self, button: &ButtonMove) -> Point {
        Point {
            x: self.x + button.x,
            y: self.y + button.y,
        }
    }
}

#[derive(Debug)]
struct ButtonMove {
    x: i128,
    y: i128,
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

fn to_the_moon(
    current_position: Point,
    a_moves: i128,
    b_moves: i128,
    a_cfg: &ButtonMove,
    b_cfg: &ButtonMove,
    prize_location: &Point,
    tried_soltions: &mut HashSet<(i128, i128)>,
    solutions: &mut Vec<(i128, i128)>,
) {
    if current_position == *prize_location {
        solutions.push((a_moves, b_moves));
        return;
    }
    if a_moves > 100 || b_moves > 100 {
        return;
    }

    if !tried_soltions.contains(&(a_moves + 1, b_moves)) {
        to_the_moon(
            current_position.press_button(a_cfg),
            a_moves + 1,
            b_moves,
            a_cfg,
            b_cfg,
            prize_location,
            tried_soltions,
            solutions,
        );
        tried_soltions.insert((a_moves + 1, b_moves));
    }
    if !tried_soltions.contains(&(a_moves, b_moves + 1)) {
        to_the_moon(
            current_position.press_button(b_cfg),
            a_moves,
            b_moves + 1,
            a_cfg,
            b_cfg,
            prize_location,
            tried_soltions,
            solutions,
        );
        tried_soltions.insert((a_moves, b_moves + 1));
    }
}

pub fn part1(input: &str) -> Answer {
    let machines = get_input(input);

    let mut result = 0;
    for machine in machines.iter() {
        let mut solutions: Vec<(i128, i128)> = Vec::new();
        let mut tried_solutions: HashSet<(i128, i128)> = HashSet::new();
        to_the_moon(
            Point { x: 0, y: 0 },
            0,
            0,
            &machine.0,
            &machine.1,
            &machine.2,
            &mut tried_solutions,
            &mut solutions,
        );

        if !solutions.is_empty() {
            let mut min_tokens = std::i128::MAX;
            for solution in solutions.iter() {
                let tokens = solution.0 * 3 + solution.1;
                if tokens < min_tokens {
                    min_tokens = tokens;
                }
            }
            result += min_tokens;
        }
    }
    result.into()
}

fn to_the_moon2(
    a_cfg: &ButtonMove,
    b_cfg: &ButtonMove,
    prize_location: &Point,
) -> Option<(i128, i128)> {
    let mut result = None;
    if (prize_location.y * a_cfg.x - prize_location.x * a_cfg.y)
        % (b_cfg.y * a_cfg.x - b_cfg.x * a_cfg.y)
        == 0
    {
        let b = (prize_location.y * a_cfg.x - prize_location.x * a_cfg.y)
            / (b_cfg.y * a_cfg.x - b_cfg.x * a_cfg.y);
        if (prize_location.y * b_cfg.x - prize_location.x * b_cfg.y)
            % (a_cfg.y * b_cfg.x - b_cfg.y * a_cfg.x)
            == 0
        {
            let a = (prize_location.y * b_cfg.x - prize_location.x * b_cfg.y)
                / (a_cfg.y * b_cfg.x - b_cfg.y * a_cfg.x);
            if a > 0 && b > 0 {
                result = Some((a, b));
            }
        }
    }
    result
}

pub fn part2(input: &str) -> Answer {
    let machines = get_input(input);

    let mut result = 0;
    for machine in machines.iter() {
        let prize_location = Point {
            x: machine.2.x + 10000000000000,
            y: machine.2.y + 10000000000000,
        };
        if let Some(solution) = to_the_moon2(&machine.0, &machine.1, &prize_location) {
            result += solution.0 * 3 + solution.1;
        }
    }
    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day13_sample"), 480i128.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day13_sample"), 480u64.into());
    }
}
