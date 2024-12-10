use crate::utils::{self, Answer};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn step(&self, map: &Map, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    None
                } else {
                    Some(Point {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
            Direction::Down => {
                if self.y == map.height - 1 {
                    None
                } else {
                    Some(Point {
                        x: self.x,
                        y: self.y + 1,
                    })
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some(Point {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
            Direction::Right => {
                if self.x == map.width - 1 {
                    None
                } else {
                    Some(Point {
                        x: self.x + 1,
                        y: self.y,
                    })
                }
            }
        }
    }
}

fn get_input(file_path: &str) -> (Map, Vec<Point>) {
    let lines = utils::read_lines(file_path);
    let width = lines[0].len();
    let height = lines.len();

    let mut map: Map = Map {
        map: Vec::new(),
        width,
        height,
    };
    let mut points: Vec<Point> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<u32> = Vec::new();
        for (x, c) in line
            .chars()
            .enumerate()
            .map(|(x, c)| (x, c.to_digit(10).unwrap_or(10)))
        {
            row.push(c);
            if c == 0 {
                points.push(Point { x, y });
            }
        }
        map.map.push(row);
    }

    (map, points)
}

fn count_trail(map: &Map, point: &Point, endings: &mut Vec<Point>) -> u32 {
    let mut count = 0;

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    {
        if let Some(next) = point.step(map, direction) {
            let value = map.map[next.y][next.x];
            let previous_value = map.map[point.y][point.x];
            if value == previous_value + 1 {
                if value == 9 {
                    if !endings.contains(&next) {
                        endings.push(next);
                        count += 1;
                    }
                } else {
                    count += count_trail(map, &next, endings);
                }
            }
        }
    }

    count
}

pub fn part1(input: &str) -> Answer {
    let (map, starts) = get_input(input);

    let mut num_trails = 0;

    for start in starts.iter() {
        let mut endings: Vec<Point> = Vec::new();
        num_trails += count_trail(&map, start, &mut endings);
    }

    num_trails.into()
}

pub fn part2(_: &str) -> Answer {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day10_sample"), 1u32.into());
        assert_eq!(part1("./inputs/day10_sample2"), 2u32.into());
        assert_eq!(part1("./inputs/day10_sample3"), 4u32.into());
        assert_eq!(part1("./inputs/day10_sample4"), 3u32.into());
        assert_eq!(part1("./inputs/day10_sample5"), 36u32.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day10_sample"), 0.into());
    }
}
