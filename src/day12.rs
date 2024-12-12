use std::char;

use crate::utils::{self, Answer};

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn check_direction(
        &self,
        matrix: &Vec<String>,
        direction: &Direction,
    ) -> Option<(char, Point)> {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    None
                } else {
                    Some((
                        matrix[self.y - 1].chars().nth(self.x).unwrap(),
                        Point::new(self.x, self.y - 1),
                    ))
                }
            }
            Direction::Down => {
                if self.y == matrix.len() - 1 {
                    None
                } else {
                    Some((
                        matrix[self.y + 1].chars().nth(self.x).unwrap(),
                        Point::new(self.x, self.y + 1),
                    ))
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some((
                        matrix[self.y].chars().nth(self.x - 1).unwrap(),
                        Point::new(self.x - 1, self.y),
                    ))
                }
            }
            Direction::Right => {
                if self.x == matrix[0].len() - 1 {
                    None
                } else {
                    Some((
                        matrix[self.y].chars().nth(self.x + 1).unwrap(),
                        Point::new(self.x + 1, self.y),
                    ))
                }
            }
        }
    }

    fn next_point(&self, direction: &Direction) -> Vec<Point> {
        match direction {
            Direction::Up => vec![
                Point::new(3 * self.x, 3 * self.y),
                Point::new(3 * self.x + 1, 3 * self.y),
                Point::new(3 * self.x + 2, 3 * self.y),
            ],

            Direction::Down => vec![
                Point::new(3 * self.x, 3 * self.y + 2),
                Point::new(3 * self.x + 1, 3 * self.y + 2),
                Point::new(3 * self.x + 2, 3 * self.y + 2),
            ],

            Direction::Left => vec![
                Point::new(3 * self.x, 3 * self.y),
                Point::new(3 * self.x, 3 * self.y + 1),
                Point::new(3 * self.x, 3 * self.y + 2),
            ],
            Direction::Right => vec![
                Point::new(3 * self.x + 2, 3 * self.y),
                Point::new(3 * self.x + 2, 3 * self.y + 1),
                Point::new(3 * self.x + 2, 3 * self.y + 2),
            ],
        }
    }
}

fn find_region(matrix: &Vec<String>, visited: &mut Vec<Vec<char>>, point: &Point) -> (u64, u64) {
    let mut area = 1;
    let mut perimeter = 0;

    visited[point.y][point.x] = matrix[point.y].chars().nth(point.x).unwrap();

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    {
        if let Some((c, next)) = point.check_direction(matrix, direction) {
            if visited[next.y][next.x] != matrix[point.y].chars().nth(point.x).unwrap() {
                if c == matrix[point.y].chars().nth(point.x).unwrap() {
                    let (new_area, new_perimeter) = find_region(matrix, visited, &next);
                    area += new_area;
                    perimeter += new_perimeter;
                } else {
                    perimeter += 1;
                }
            }
        } else {
            perimeter += 1;
        }
    }

    (area, perimeter)
}

pub fn part1(input: &str) -> Answer {
    let matrix = utils::read_lines(input);
    let mut visited: Vec<Vec<char>> = matrix.iter().map(|x| vec![' '; x.len()]).collect();
    let mut result = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if visited[y][x] == ' ' {
                let (area, perimeter) = find_region(&matrix, &mut visited, &Point::new(x, y));
                result += area * perimeter;
            }
        }
    }
    result.into()
}

fn find_region2(
    matrix: &Vec<String>,
    visited: &mut Vec<Vec<char>>,
    new_matrix: &mut Vec<Vec<char>>,
    point: &Point,
) -> (u64, u64) {
    let mut area = 1;
    let mut perimeter = 0;

    visited[point.y][point.x] = matrix[point.y].chars().nth(point.x).unwrap();

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    {
        if let Some((c, next)) = point.check_direction(matrix, direction) {
            if visited[next.y][next.x] != matrix[point.y].chars().nth(point.x).unwrap() {
                if c == matrix[point.y].chars().nth(point.x).unwrap() {
                    let (new_area, new_perimeter) =
                        find_region2(matrix, visited, new_matrix, &next);
                    area += new_area;
                    perimeter += new_perimeter;
                } else {
                    perimeter += 1;
                    let next_points = point.next_point(direction);
                    for next_point in next_points {
                        if *direction == Direction::Up || *direction == Direction::Down {
                            if new_matrix[next_point.y][next_point.x] == '|' {
                                new_matrix[next_point.y][next_point.x] = '/';
                            } else {
                                new_matrix[next_point.y][next_point.x] = '-';
                            }
                        } else {
                            if new_matrix[next_point.y][next_point.x] == '-' {
                                new_matrix[next_point.y][next_point.x] = '/';
                            } else {
                                new_matrix[next_point.y][next_point.x] = '|';
                            }
                        }
                    }
                }
            }
        } else {
            perimeter += 1;
            let next_points = point.next_point(direction);
            for next_point in next_points {
                if *direction == Direction::Up || *direction == Direction::Down {
                    if new_matrix[next_point.y][next_point.x] == '|' {
                        new_matrix[next_point.y][next_point.x] = '/';
                    } else {
                        new_matrix[next_point.y][next_point.x] = '-';
                    }
                } else {
                    if new_matrix[next_point.y][next_point.x] == '-' {
                        new_matrix[next_point.y][next_point.x] = '/';
                    } else {
                        new_matrix[next_point.y][next_point.x] = '|';
                    }
                }
            }
        }
    }

    (area, perimeter)
}

pub fn part2(input: &str) -> Answer {
    let matrix = utils::read_lines(input);
    let mut visited: Vec<Vec<char>> = matrix.iter().map(|x| vec![' '; x.len()]).collect();
    let mut result = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            let mut new_matrix: Vec<Vec<char>> =
                vec![vec![' '; matrix[0].len() * 3]; matrix.len() * 3];
            if visited[y][x] == ' ' {
                let (area, _) =
                    find_region2(&matrix, &mut visited, &mut new_matrix, &Point::new(x, y));
                let mut horizontal = false;
                let mut vertical = false;
                let mut sides = 0;
                for line in new_matrix.iter() {
                    for c in line.iter() {
                        match *c {
                            '-' | '/' => {
                                if !horizontal {
                                    horizontal = true;
                                    sides += 1;
                                }
                            }
                            _ => {
                                horizontal = false;
                            }
                        }
                    }
                }

                for col in 0..new_matrix[0].len() {
                    for row in 0..new_matrix.len() {
                        match new_matrix[row][col] {
                            '|' | '/' => {
                                if !vertical {
                                    vertical = true;
                                    sides += 1;
                                }
                            }
                            _ => {
                                vertical = false;
                            }
                        }
                    }
                }
                result += area * sides;
            }
        }
    }
    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day12_sample"), 140u64.into());
        assert_eq!(part1("./inputs/day12_sample2"), 772u64.into());
        assert_eq!(part1("./inputs/day12_sample3"), 1930u64.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day12_sample"), 80u64.into());
        assert_eq!(part2("./inputs/day12_sample2"), 436u64.into());
        assert_eq!(part2("./inputs/day12_sample3"), 1206u64.into());
        assert_eq!(part2("./inputs/day12_sample4"), 236u64.into());
        assert_eq!(part2("./inputs/day12_sample5"), 368u64.into());
    }
}
