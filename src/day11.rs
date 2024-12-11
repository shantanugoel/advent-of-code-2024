use std::collections::HashMap;

use crate::utils::{self, Answer};

fn get_input(input: &str) -> Vec<u64> {
    let lines = utils::read_lines(input);
    lines[0].split(" ").map(|x| x.parse().unwrap()).collect()
}

fn blink(stone: &u64, num_blinks: u32, memory: &mut HashMap<(u64, u32), u64>) -> u64 {
    let mut new_stones = Vec::new();
    let mut count = 0;

    if memory.contains_key(&(*stone, num_blinks)) {
        return memory[&(*stone, num_blinks)];
    }
    if *stone == 0 {
        new_stones.push(1);
    } else if stone.to_string().len() % 2 == 0 {
        let left = &stone.to_string()[0..stone.to_string().len() / 2];
        let right = &stone.to_string()[stone.to_string().len() / 2..stone.to_string().len()];
        new_stones.push(left.parse().unwrap());
        new_stones.push(right.parse().unwrap());
    } else {
        new_stones.push(stone * 2024);
    }

    memory.insert((*stone, 1), new_stones.len() as u64);

    if num_blinks == 1 {
        count = new_stones.len() as u64;
    } else {
        for new_stone in new_stones.iter() {
            let temp = blink(new_stone, num_blinks - 1, memory);
            memory.insert((*new_stone, num_blinks - 1), temp);
            count += temp;
        }
    }

    count
}

pub fn parts(input: &str, num_blinks: u32) -> Answer {
    let stones = get_input(input);
    let mut count = 0;

    for stone in stones.iter() {
        count += blink(stone, num_blinks, &mut HashMap::new());
    }

    count.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(parts("./inputs/day11_sample2", 25), 55312u64.into());
    }
}
