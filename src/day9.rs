use crate::utils::{self, Answer};

pub fn part1(input: &str) -> Answer {
    let data = utils::read_line(input);

    let mut back = data.len() - 1;
    let mut last_file_id = back / 2 + data.len() % 2;
    let mut back_data = 0;
    let mut current_id = 0;
    let mut fs_index = 0;
    let mut checksum: u64 = 0;
    for (front, x) in data
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
    {
        if back < front {
            break;
        }
        let number;
        if back == front {
            number = back_data;
        } else {
            number = x;
        }
        for _ in (0..number).rev() {
            if front % 2 == 0 {
                checksum += fs_index * current_id;
            } else {
                if back_data == 0 {
                    last_file_id -= 1;
                    if back % 2 != 0 {
                        back -= 1;
                    }
                    if back < front {
                        break;
                    }
                    back_data = data.chars().nth(back).unwrap().to_digit(10).unwrap();
                }
                checksum += fs_index * last_file_id as u64;
                back_data -= 1;
                if back_data == 0 {
                    back -= 1;
                }
            }
            fs_index += 1;
        }
        if front % 2 == 0 {
            current_id += 1;
        }
    }

    checksum.into()
}

pub fn part2(input: &str) -> Answer {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day9_sample"), 1928.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day9_sample"), 34.into());
    }
}
