use crate::utils::{self, Answer};

fn get_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let lines = utils::read_lines(input);
    let mut result: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in lines.iter() {
        let (a, b) = line.split_once(':').unwrap();
        let numbers = b.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        result.push((a.parse().unwrap(), numbers));
    }
    result
}

enum Operation {
    Add,
    Mul,
}

fn try_combination(
    input: &Vec<u64>,
    initial: u64,
    next_position: usize,
    target: u64,
    op: Operation,
) -> bool {
    let mut result = false;
    if next_position < input.len() {
        let value = match op {
            Operation::Add => initial + input[next_position],
            Operation::Mul => initial * input[next_position],
        };
        if try_combination(input, value, next_position + 1, target, Operation::Add) {
            result = true;
        } else if try_combination(input, value, next_position + 1, target, Operation::Mul) {
            result = true;
        }
    } else {
        if initial == target {
            result = true;
        }
    }
    result
}

pub fn part1(input: &str) -> Answer {
    let lines = get_input(input);
    let mut result = 0;

    for line in lines.iter() {
        if try_combination(&line.1, line.1[0], 1, line.0, Operation::Add) {
            result += line.0;
        } else if try_combination(&line.1, line.1[0], 1, line.0, Operation::Mul) {
            result += line.0;
        }
    }

    result.into()
}

pub fn part2(_: &str) -> Answer {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day7_sample"), 3749u64.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day7_sample"), 6.into());
    }
}
