use crate::utils::{self, Answer};

fn get_input(input: &str) -> Vec<u64> {
    let lines = utils::read_lines(input);
    lines[0].split(" ").map(|x| x.parse().unwrap()).collect()
}

pub fn parts(input: &str, num_blinks: u32) -> Answer {
    let mut stones = get_input(input);

    for _ in 0..num_blinks {
        let mut new_stones = Vec::new();
        for stone in stones.iter() {
            let stone_string = stone.to_string();
            if *stone == 0 {
                new_stones.push(1);
            } else if stone_string.len() % 2 == 0 {
                let left = &stone_string[0..stone_string.len() / 2];
                let right = &stone_string[stone_string.len() / 2..stone_string.len()];
                new_stones.push(left.parse().unwrap());
                new_stones.push(right.parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    stones.len().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(parts("./inputs/day11_sample2", 25), 55312i32.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(parts("./inputs/day11_sample", 75), 2713310158u64.into());
    }
}
