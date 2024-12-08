use std::collections::{HashMap, HashSet};

use crate::utils::{self, Answer};

#[derive(Debug)]
struct NodeMap {
    map: HashMap<char, Vec<(usize, usize)>>,
}

impl NodeMap {
    fn new() -> NodeMap {
        NodeMap {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: char, value: (usize, usize)) {
        self.map.entry(key).or_default().push(value);
    }
}

fn get_input(input: &str) -> (NodeMap, usize, usize) {
    let lines = utils::read_lines(input);
    let mut result = NodeMap::new();
    for (y, line) in lines.iter().enumerate() {
        line.char_indices()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, c)| {
                result.insert(c, (x, y));
            });
    }
    (result, lines[0].len(), lines.len())
}

pub fn part1(input: &str) -> Answer {
    let (antennas, x_max, y_max) = get_input(input);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for antenna in antennas.map.values() {
        for i in 0..antenna.len() {
            for j in i + 1..antenna.len() {
                let (x1, y1) = antenna[i];
                let (x2, y2) = antenna[j];
                let x_diff: i32 = x1 as i32 - x2 as i32;
                let y_diff: i32 = y1 as i32 - y2 as i32;
                let x_new_1 = x1 as i32 + x_diff;
                let y_new_1 = y1 as i32 + y_diff;
                let x_new_2 = x2 as i32 - x_diff;
                let y_new_2 = y2 as i32 - y_diff;
                if x_new_1 >= 0
                    && x_new_1 < x_max as i32
                    && y_new_1 >= 0
                    && y_new_1 < y_max as i32
                    && !antinodes.contains(&(x_new_1 as usize, y_new_1 as usize))
                {
                    antinodes.insert((x_new_1 as usize, y_new_1 as usize));
                }
                if x_new_2 >= 0
                    && x_new_2 < x_max as i32
                    && y_new_2 >= 0
                    && y_new_2 < y_max as i32
                    && !antinodes.contains(&(x_new_2 as usize, y_new_2 as usize))
                {
                    antinodes.insert((x_new_2 as usize, y_new_2 as usize));
                }
            }
        }
    }
    antinodes.len().into()
}

pub fn part2(input: &str) -> Answer {
    let (antennas, x_max, y_max) = get_input(input);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for antenna in antennas.map.values() {
        for i in 0..antenna.len() {
            for j in i + 1..antenna.len() {
                let (x1, y1) = antenna[i];
                let (x2, y2) = antenna[j];
                let x_diff: i32 = x1 as i32 - x2 as i32;
                let y_diff: i32 = y1 as i32 - y2 as i32;

                let flips = [true, false];
                for &flip in flips.iter() {
                    let mut x_new;
                    let mut y_new;
                    if flip {
                        x_new = x1 as i32;
                        y_new = y1 as i32;
                    } else {
                        x_new = x2 as i32;
                        y_new = y2 as i32;
                    }
                    loop {
                        if flip {
                            x_new = x_new + x_diff;
                            y_new = y_new + y_diff;
                        } else {
                            x_new = x_new - x_diff;
                            y_new = y_new - y_diff;
                        }

                        if x_new < 0 || x_new >= x_max as i32 || y_new < 0 || y_new >= y_max as i32
                        {
                            break;
                        }
                        if !antinodes.contains(&(x_new as usize, y_new as usize)) {
                            antinodes.insert((x_new as usize, y_new as usize));
                        }
                    }
                }
                if !antinodes.contains(&(x1, y1)) {
                    antinodes.insert((x1, y1));
                }
                if !antinodes.contains(&(x2, y2)) {
                    antinodes.insert((x2, y2));
                }
            }
        }
    }
    antinodes.len().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day8_sample"), 14.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day8_sample"), 34.into());
    }
}
