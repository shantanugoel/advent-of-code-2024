use std::{collections::HashMap, vec};

use crate::utils::{self, Answer};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug)]
struct Maze {
    height: usize,
    map: Vec<Vec<char>>,
    end_position: (usize, usize),
}

impl Maze {
    fn step(
        &self,
        deer_position: (usize, usize),
        deer_direction: &Direction,
    ) -> Option<(usize, usize)> {
        let (x, y);
        match deer_direction {
            Direction::Up => {
                x = deer_position.0;
                y = deer_position.1 - 1;
            }
            Direction::Down => {
                x = deer_position.0;
                y = deer_position.1 + 1;
            }
            Direction::Left => {
                x = deer_position.0 - 1;
                y = deer_position.1;
            }
            Direction::Right => {
                x = deer_position.0 + 1;
                y = deer_position.1;
            }
        }
        if self.map[y][x] != '#' {
            Some((x, y))
        } else {
            None
        }
    }
}

fn get_input(input: &str) -> Maze {
    let lines = utils::read_lines(input);
    let width = lines[0].len();
    let height = lines.len();
    let end_position = (width - 2, 1);

    let mut map: Vec<Vec<char>> = Vec::new();
    for row in lines.iter() {
        let mut new_row = Vec::new();
        for col in row.chars() {
            new_row.push(col);
        }
        map.push(new_row);
    }

    Maze {
        height,
        map,
        end_position,
    }
}

fn parse(
    maze: &Maze,
    deer_position: (usize, usize),
    deer_direction: Direction,
    all_scores: &mut Vec<u64>,
) {
    let mut positions_to_try: Vec<(usize, usize, Direction, u64)> = Vec::new();
    let mut positions_tried: HashMap<(usize, usize, Direction), u64> = HashMap::new();
    positions_to_try.push((deer_position.0, deer_position.1, deer_direction, 0));
    loop {
        if positions_to_try.is_empty() {
            break;
        }
        let (position_x, position_y, old_direction, score) = positions_to_try.pop().unwrap();
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if direction == old_direction.opposite() {
                continue;
            }
            let new_score;
            if direction == old_direction {
                new_score = score + 1;
            } else {
                new_score = score + 1001;
            }
            if let Some(new_position) = maze.step((position_x, position_y), &direction) {
                if new_position != maze.end_position {
                    if !positions_tried.contains_key(&(new_position.0, new_position.1, direction))
                        || positions_tried[&(new_position.0, new_position.1, direction)] > new_score
                    {
                        positions_to_try.push((
                            new_position.0,
                            new_position.1,
                            direction,
                            new_score,
                        ));
                    }
                } else {
                    all_scores.push(new_score);
                }
            }
            if !positions_tried.contains_key(&(position_x, position_y, direction))
                || positions_tried[&(position_x, position_y, direction)] > score
            {
                positions_tried.insert((position_x, position_y, direction), score);
            }
        }
    }
}

pub fn part1(input: &str) -> Answer {
    let maze = get_input(input);
    let mut all_scores: Vec<u64> = Vec::new();
    parse(
        &maze,
        (1, maze.height - 2),
        Direction::Right,
        &mut all_scores,
    );
    all_scores.sort_unstable();
    all_scores[0].into()
}

fn parse2(
    maze: &Maze,
    deer_position: (usize, usize),
    deer_direction: Direction,
    all_scores: &mut Vec<u64>,
    positions_tried: &mut HashMap<(usize, usize, Direction), u64>,
) {
    let mut positions_to_try: Vec<(usize, usize, Direction, u64)> = Vec::new();
    positions_to_try.push((deer_position.0, deer_position.1, deer_direction, 0));
    loop {
        if positions_to_try.is_empty() {
            break;
        }
        let (position_x, position_y, old_direction, score) = positions_to_try.pop().unwrap();
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if direction == old_direction.opposite() {
                continue;
            }
            let new_score;
            if direction == old_direction {
                new_score = score + 1;
            } else {
                new_score = score + 1001;
            }
            if let Some(new_position) = maze.step((position_x, position_y), &direction) {
                if new_position != maze.end_position {
                    if !positions_tried.contains_key(&(new_position.0, new_position.1, direction))
                        || positions_tried[&(new_position.0, new_position.1, direction)] > new_score
                    {
                        positions_to_try.push((
                            new_position.0,
                            new_position.1,
                            direction,
                            new_score,
                        ));
                    }
                } else {
                    all_scores.push(new_score);
                }
            }
            if !positions_tried.contains_key(&(position_x, position_y, direction))
                || positions_tried[&(position_x, position_y, direction)] > new_score
            {
                positions_tried.insert((position_x, position_y, direction), new_score);
            }
        }
    }
}

pub fn part2(input: &str) -> Answer {
    let maze = get_input(input);
    let mut all_scores: Vec<u64> = Vec::new();
    let mut positions_tried: HashMap<(usize, usize, Direction), u64> = HashMap::new();
    parse2(
        &maze,
        (1, maze.height - 2),
        Direction::Right,
        &mut all_scores,
        &mut positions_tried,
    );
    all_scores.sort_unstable();
    let lowest = all_scores[0] + 1;

    let mut positions_to_try: Vec<(usize, usize, u64)> =
        vec![(maze.end_position.0, maze.end_position.1, lowest)];

    let mut maze_map: Vec<Vec<i32>> = vec![vec![0; maze.map[0].len()]; maze.height];

    loop {
        if positions_to_try.is_empty() {
            break;
        }
        let (position_x, position_y, score) = positions_to_try.pop().unwrap();

        let mut new_positions_to_try: Vec<(usize, usize, u64)> = Vec::new();
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(new_position) = maze.step((position_x, position_y), &direction) {
                if new_position != (1, maze.height - 2)
                    && positions_tried.contains_key(&(
                        new_position.0,
                        new_position.1,
                        direction.opposite(),
                    ))
                {
                    let position_score =
                        positions_tried[&(new_position.0, new_position.1, direction.opposite())];
                    if position_score < score {
                        new_positions_to_try.push((new_position.0, new_position.1, position_score));
                    }
                }
            }
        }

        for position in new_positions_to_try {
            maze_map[position.1][position.0] += 1;
            positions_to_try.push(position);
        }
    }

    let mut count = 2;
    for row in maze_map.iter() {
        for col in row.iter() {
            if *col > 0 {
                count += 1;
            }
        }
    }

    count.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day16_sample"), 7036u64.into());
        assert_eq!(part1("./inputs/day16_sample2"), 11048u64.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day16_sample"), 45i32.into());
        assert_eq!(part2("./inputs/day16_sample2"), 64i32.into());
    }
}
