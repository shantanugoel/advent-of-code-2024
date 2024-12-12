use crate::utils::{self, Answer};

#[derive(Debug)]
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
}

fn find_region(matrix: &Vec<String>, visited: &mut Vec<Vec<char>>, point: &Point) -> (u64, u64) {
    let mut area = 1;
    let mut perimeter = 0;

    // println!("{}", matrix[point.y].chars().nth(point.x).unwrap(),);
    visited[point.y][point.x] = matrix[point.y].chars().nth(point.x).unwrap();

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    {
        // println!("{:?}", direction);
        if let Some((c, next)) = point.check_direction(matrix, direction) {
            if visited[next.y][next.x] != matrix[point.y].chars().nth(point.x).unwrap() {
                if c == matrix[point.y].chars().nth(point.x).unwrap() {
                    // visited[next.y][next.x] = true;
                    let (new_area, new_perimeter) = find_region(matrix, visited, &next);
                    area += new_area;
                    perimeter += new_perimeter;
                    // println!("{} {}", new_area, new_perimeter);
                } else {
                    perimeter += 1;
                }
            }
        } else {
            // println!("Not found");
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
                // println!(
                //     "{} {} {}",
                //     matrix[y].chars().nth(x).unwrap(),
                //     area,
                //     perimeter
                // );
            }
        }
    }
    result.into()
}

pub fn part2(input: &str) -> Answer {
    todo!()
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
        assert_eq!(part2(""), 0.into());
    }
}
