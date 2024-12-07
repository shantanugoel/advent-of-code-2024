use crate::utils::{self, Answer};

fn get_input(input: &str) -> Vec<(u128, Vec<u128>)> {
    let lines = utils::read_lines(input);
    let mut result: Vec<(u128, Vec<u128>)> = Vec::new();
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
    Concat,
}

fn try_combination(
    input: &Vec<u128>,
    initial: u128,
    next_position: usize,
    target: u128,
    op: Operation,
) -> bool {
    let mut result = false;
    if next_position < input.len() {
        let value = match op {
            Operation::Add => initial + input[next_position],
            Operation::Mul => initial * input[next_position],
            _ => 0,
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

fn try_combination_2(
    input: &Vec<u128>,
    initial: u128,
    next_position: usize,
    target: u128,
    op: Operation,
) -> bool {
    let mut result = false;
    if next_position < input.len() {
        let value = match op {
            Operation::Add => initial + input[next_position],
            Operation::Mul => initial * input[next_position],
            Operation::Concat => {
                let mut temp = initial;
                let mut temp_next = input[next_position];
                while temp_next > 0 {
                    temp = temp * 10;
                    temp_next = temp_next / 10;
                }
                temp + input[next_position]
            }
        };
        if try_combination_2(input, value, next_position + 1, target, Operation::Add) {
            result = true;
        } else if try_combination_2(input, value, next_position + 1, target, Operation::Mul) {
            result = true;
        } else if try_combination_2(input, value, next_position + 1, target, Operation::Concat) {
            result = true;
        }
    } else {
        if initial == target {
            result = true;
        }
    }
    result
}

pub fn part2(input: &str) -> Answer {
    let lines = get_input(input);
    let mut result = 0;

    for line in lines.iter() {
        if try_combination_2(&line.1, line.1[0], 1, line.0, Operation::Add) {
            result += line.0;
        } else if try_combination_2(&line.1, line.1[0], 1, line.0, Operation::Mul) {
            result += line.0;
        } else if try_combination_2(&line.1, line.1[0], 1, line.0, Operation::Concat) {
            result += line.0;
        }
    }

    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day7_sample"), 3749u128.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day7_sample"), 11387u128.into());
    }
}
