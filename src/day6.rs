use crate::utils::{self, Answer};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Layout {
    layout: Vec<String>,
    guard_position: (usize, usize),
    guard_direction: Direction,
}

impl Layout {
    fn step(&mut self) -> Option<usize> {
        let mut within_bounds = true;
        let mut count = 0;
        match self.guard_direction {
            Direction::Up => {
                if self.guard_position.1 == 0 {
                    within_bounds = false
                } else {
                    if self.layout[self.guard_position.1 - 1]
                        .chars()
                        .nth(self.guard_position.0)
                        .unwrap()
                        == '#'
                    {
                        self.guard_direction = Direction::Right;
                    } else {
                        if self.layout[self.guard_position.1]
                            .chars()
                            .nth(self.guard_position.0)
                            .unwrap()
                            != 'X'
                        {
                            self.layout[self.guard_position.1].replace_range(
                                self.guard_position.0..self.guard_position.0 + 1,
                                "X",
                            );
                            count += 1;
                        }
                        self.guard_position.1 -= 1;
                    }
                }
            }
            Direction::Down => {
                if self.guard_position.1 == self.layout.len() - 1 {
                    within_bounds = false
                } else {
                    if self.layout[self.guard_position.1 + 1]
                        .chars()
                        .nth(self.guard_position.0)
                        .unwrap()
                        == '#'
                    {
                        self.guard_direction = Direction::Left;
                    } else {
                        if self.layout[self.guard_position.1]
                            .chars()
                            .nth(self.guard_position.0)
                            .unwrap()
                            != 'X'
                        {
                            self.layout[self.guard_position.1].replace_range(
                                self.guard_position.0..self.guard_position.0 + 1,
                                "X",
                            );
                            count += 1;
                        }
                        self.guard_position.1 += 1;
                    }
                }
            }
            Direction::Left => {
                if self.guard_position.0 == 0 {
                    within_bounds = false
                } else {
                    if self.layout[self.guard_position.1]
                        .chars()
                        .nth(self.guard_position.0 - 1)
                        .unwrap()
                        == '#'
                    {
                        self.guard_direction = Direction::Up;
                    } else {
                        if self.layout[self.guard_position.1]
                            .chars()
                            .nth(self.guard_position.0)
                            .unwrap()
                            != 'X'
                        {
                            self.layout[self.guard_position.1].replace_range(
                                self.guard_position.0..self.guard_position.0 + 1,
                                "X",
                            );
                            count += 1;
                        }
                        self.guard_position.0 -= 1;
                    }
                }
            }
            Direction::Right => {
                if self.guard_position.0 == self.layout.len() - 1 {
                    within_bounds = false
                } else {
                    if self.layout[self.guard_position.1]
                        .chars()
                        .nth(self.guard_position.0 + 1)
                        .unwrap()
                        == '#'
                    {
                        self.guard_direction = Direction::Down;
                    } else {
                        if self.layout[self.guard_position.1]
                            .chars()
                            .nth(self.guard_position.0)
                            .unwrap()
                            != 'X'
                        {
                            self.layout[self.guard_position.1].replace_range(
                                self.guard_position.0..self.guard_position.0 + 1,
                                "X",
                            );
                            count += 1;
                        }
                        self.guard_position.0 += 1;
                    }
                }
            }
        }
        if within_bounds {
            Some(count)
        } else {
            None
        }
    }
}

fn get_input(file_path: &str) -> Layout {
    let lines = utils::read_lines(file_path);
    let mut position_x = 0;
    let mut position_y = 0;
    let mut direction = Direction::Up;
    for (row, line) in lines.iter().enumerate() {
        position_y = row;
        if let Some(col) = line.find('^') {
            position_x = col;
            direction = Direction::Up;
            break;
        } else if let Some(col) = line.find('v') {
            position_x = col;
            direction = Direction::Down;
            break;
        } else if let Some(col) = line.find('<') {
            position_x = col;
            direction = Direction::Left;
            break;
        } else if let Some(col) = line.find('>') {
            position_x = col;
            direction = Direction::Right;
            break;
        }
    }

    Layout {
        layout: lines,
        guard_position: (position_x, position_y),
        guard_direction: direction,
    }
}

pub fn part1(input: &str) -> Answer {
    let mut input = get_input(input);
    let mut sum = 1;

    loop {
        if let Some(count) = input.step() {
            sum += count;
        } else {
            break;
        }
    }

    sum.into()
}

pub fn part2(input: &str) -> Answer {
    0.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = part1("./inputs/day6_sample");
        assert_eq!(answer, 41.into());
    }
}
