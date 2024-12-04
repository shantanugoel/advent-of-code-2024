use crate::utils::{self, Answer};

fn get_input() -> Vec<String> {
    utils::read_lines("./inputs/day4")
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagonalUpForward,
    DiagonalUpBackward,
    DiagonalDownForward,
    DiagonalDownBackward,
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
    rows: usize,
    cols: usize,
}

impl Coordinates {
    fn step(self, direction: &Direction) -> Option<Self> {
        let mut target = self.clone();
        let updated = match direction {
            Direction::Up => {
                if target.y == 0 {
                    false
                } else {
                    target.y -= 1;
                    true
                }
            }
            Direction::Down => {
                if target.y == target.rows - 1 {
                    false
                } else {
                    target.y += 1;
                    true
                }
            }
            Direction::Left => {
                if target.x == 0 {
                    false
                } else {
                    target.x -= 1;
                    true
                }
            }
            Direction::Right => {
                if target.x == target.cols - 1 {
                    false
                } else {
                    target.x += 1;
                    true
                }
            }
            Direction::DiagonalUpForward => {
                if target.y == 0 || target.x == target.cols - 1 {
                    false
                } else {
                    target.y -= 1;
                    target.x += 1;
                    true
                }
            }
            Direction::DiagonalUpBackward => {
                if target.y == 0 || target.x == 0 {
                    false
                } else {
                    target.y -= 1;
                    target.x -= 1;
                    true
                }
            }
            Direction::DiagonalDownForward => {
                if target.y == target.rows - 1 || target.x == target.cols - 1 {
                    false
                } else {
                    target.y += 1;
                    target.x += 1;
                    true
                }
            }
            Direction::DiagonalDownBackward => {
                if target.y == target.rows - 1 || target.x == 0 {
                    false
                } else {
                    target.y += 1;
                    target.x -= 1;
                    true
                }
            }
        };
        if updated {
            Some(target)
        } else {
            None
        }
    }
}

fn search(input: Vec<String>, direction: &Direction, coords: Coordinates) -> bool {
    if let Some(next) = coords.step(direction) {
        if input[next.y].chars().nth(next.x).unwrap() == 'M' {
            if let Some(next) = next.step(direction) {
                if input[next.y].chars().nth(next.x).unwrap() == 'A' {
                    if let Some(next) = next.step(direction) {
                        if input[next.y].chars().nth(next.x).unwrap() == 'S' {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

pub fn part1() -> Answer {
    let lines = get_input();
    let cols = lines[0].len();
    let rows = lines.len();

    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            if lines[row].chars().nth(col).unwrap() != 'X' {
                continue;
            }
            let coords = Coordinates {
                x: col,
                y: row,
                rows,
                cols,
            };
            if search(lines.clone(), &Direction::Up, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::Down, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::Left, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::Right, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalUpForward, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalUpBackward, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalDownForward, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalDownBackward, coords) {
                result += 1;
            }
        }
    }
    result.into()
}

pub fn part2() -> Answer {
    let lines = get_input();
    let cols = lines[0].len();
    let rows = lines.len();

    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            if lines[row].chars().nth(col).unwrap() != 'A' {
                continue;
            }
            let coords = Coordinates {
                x: col,
                y: row,
                rows,
                cols,
            };
            if let Some(left_top) = coords.step(&Direction::DiagonalUpBackward) {
                if let Some(left_bottom) = coords.step(&Direction::DiagonalDownBackward) {
                    if let Some(right_top) = coords.step(&Direction::DiagonalUpForward) {
                        if let Some(right_bottom) = coords.step(&Direction::DiagonalDownForward) {
                            let mas_1 = format!(
                                "{}{}",
                                lines[left_top.y].chars().nth(left_top.x).unwrap(),
                                lines[right_bottom.y].chars().nth(right_bottom.x).unwrap()
                            );
                            let mas_2 = format!(
                                "{}{}",
                                lines[right_top.y].chars().nth(right_top.x).unwrap(),
                                lines[left_bottom.y].chars().nth(left_bottom.x).unwrap()
                            );
                            if (mas_1 == "MS" || mas_1 == "SM") && (mas_2 == "MS" || mas_2 == "SM")
                            {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    result.into()
}
