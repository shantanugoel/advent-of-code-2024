use crate::utils::{self, Answer};

struct Warehouse {
    layout: Vec<Vec<char>>,
    robot_position: (usize, usize),
}

impl Warehouse {
    fn new(layout: Vec<Vec<char>>, robot_position: (usize, usize)) -> Self {
        Self {
            layout,
            robot_position,
        }
    }

    fn step(&mut self, direction: char) {
        let (mut x, mut y) = (0, 0);
        match direction {
            '^' => y = -1,
            'v' => y = 1,
            '>' => x = 1,
            '<' => x = -1,
            _ => unreachable!(),
        }

        let mut new_x = self.robot_position.0 as i32;
        let mut new_y = self.robot_position.1 as i32;
        let mut ctr: i32 = 0;
        loop {
            new_x += x;
            new_y += y;
            if self.layout[new_y as usize][new_x as usize] == '#' {
                break;
            } else if self.layout[new_y as usize][new_x as usize] != '.' {
                ctr += 1;
                continue;
            } else {
                for i in 0..ctr + 1 {
                    self.layout[(new_y - i * y) as usize][(new_x - i * x) as usize] =
                        self.layout[(new_y - (i + 1) * y) as usize][(new_x - (i + 1) * x) as usize];
                }
                self.layout[self.robot_position.1][self.robot_position.0] = '.';
                self.robot_position.0 = (self.robot_position.0 as i32 + x) as usize;
                self.robot_position.1 = (self.robot_position.1 as i32 + y) as usize;
                break;
            }
        }
    }
}

fn get_input(input: &str) -> (Warehouse, Vec<char>) {
    let mut layout: Vec<Vec<char>> = Vec::new();
    let lines = utils::read_lines(input);
    let mut robot_position = (0, 0);

    let mut movements: Vec<char> = Vec::new();
    let mut fill_movements = false;
    for (row, line) in lines.iter().enumerate() {
        if line.is_empty() {
            fill_movements = true;
            continue;
        }
        if !fill_movements {
            let mut line_chars: Vec<char> = Vec::new();
            for (col, c) in line.chars().enumerate() {
                if c == '@' {
                    robot_position = (col, row);
                }
                line_chars.push(c);
            }
            layout.push(line_chars);
        } else {
            movements.extend(line.chars());
        }
    }

    (Warehouse::new(layout, robot_position), movements)
}

pub fn part1(input: &str) -> Answer {
    let (mut warehouse, movements) = get_input(input);

    for movement in movements.iter() {
        warehouse.step(*movement);
        // for row in warehouse.layout.iter() {
        //     println!("{}", row.iter().collect::<String>());
        // }
    }

    let mut result: u64 = 0;
    for (y, row) in warehouse.layout.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'O' {
                result += (100 * y as u64) + x as u64;
            }
        }
    }

    result.into()
}

pub fn part2(input: &str) -> Answer {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day15_sample2"), 2028u64.into());
        assert_eq!(part1("./inputs/day15_sample"), 10092u64.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day15_sample"), 315.into());
    }
}
