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
    let data = utils::read_line(input);
    let mut checksum = 0;

    // Vec<(file_id, file_size)>
    let mut files: Vec<(u64, u64)> = Vec::new();
    let mut spaces: Vec<(u64, Vec<(u64, u64)>)> = Vec::new();

    for (index, data) in data
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
    {
        if index % 2 == 0 {
            files.push((index as u64 / 2, data as u64));
        } else {
            spaces.push((data as u64, Vec::new()));
        }
    }

    let num_files = files.len();
    for (file_index, file) in files.iter_mut().rev().enumerate() {
        for (index, space) in spaces.iter_mut().enumerate() {
            if index >= num_files - file_index - 1 {
                break;
            }
            if space.0 >= file.1 {
                space.1.push(file.clone());
                file.0 = 0;
                space.0 -= file.1;
                break;
            }
        }
    }

    let mut current_index = 0;
    let mut i = 0;
    files.iter().zip(spaces.iter()).for_each(|(file, space)| {
        i += 1;
        for _ in 0..file.1 {
            checksum += file.0 * current_index;
            current_index += 1;
        }
        for space in space.1.iter() {
            for _ in 0..space.1 {
                checksum += space.0 * current_index;
                current_index += 1;
            }
        }
        for _ in 0..space.0 {
            current_index += 1;
        }
    });

    checksum.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./inputs/day9_sample"), 1928u64.into());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./inputs/day9_sample"), 2858u64.into());
    }
}
