use regex::Regex;

use crate::utils::{self, Answer};

struct Point {
    x: i32,
    y: i32,
}

struct Velocity {
    x: i32,
    y: i32,
}

struct Robot {
    position: Point,
    velocity: Velocity,
}

fn get_input(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    let lines = utils::read_lines(input);
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in lines.iter() {
        let captures = regex.captures(line).unwrap();
        let position = Point {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        };
        let velocity = Velocity {
            x: captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap(),
        };
        robots.push(Robot { position, velocity });
    }
    robots
}

pub fn part1(input: &str, lab_width: i32, lab_height: i32) -> Answer {
    let mut robots = get_input(input);
    let time_limit = 100;

    for robot in robots.iter_mut() {
        let x_movement = (robot.velocity.x * time_limit) % lab_width;
        let y_movement = (robot.velocity.y * time_limit) % lab_height;
        if robot.velocity.x > 0 {
            robot.position.x = (robot.position.x + x_movement) % lab_width;
        } else {
            robot.position.x = (robot.position.x + x_movement + lab_width) % lab_width;
        }
        if robot.velocity.y > 0 {
            robot.position.y = (robot.position.y + y_movement) % lab_height;
        } else {
            robot.position.y = (robot.position.y + y_movement + lab_height) % lab_height;
        }
    }

    let mut quadrant_1_count = 0;
    let mut quadrant_2_count = 0;
    let mut quadrant_3_count = 0;
    let mut quadrant_4_count = 0;
    for robot in robots.iter() {
        if robot.position.x <= (lab_width / 2 - lab_width % 2)
            && robot.position.y <= (lab_height / 2 - lab_height % 2)
        {
            quadrant_1_count += 1;
        } else if robot.position.x <= (lab_width / 2 - lab_width % 2)
            && robot.position.y >= (lab_height / 2 + lab_height % 2)
        {
            quadrant_2_count += 1;
        } else if robot.position.x >= (lab_width / 2 + lab_width % 2)
            && robot.position.y >= (lab_height / 2 + lab_height % 2)
        {
            quadrant_3_count += 1;
        } else if robot.position.x >= (lab_width / 2 + lab_width % 2)
            && robot.position.y <= (lab_height / 2 - lab_height % 2)
        {
            quadrant_4_count += 1;
        }
    }

    (quadrant_1_count as u64
        * quadrant_2_count as u64
        * quadrant_3_count as u64
        * quadrant_4_count as u64)
        .into()
}

pub fn part2(input: &str, lab_width: i32, lab_height: i32) -> Answer {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day14_sample", 11, 7), 12u64.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day14_sample", 11, 7), 0.into());
    }
}
