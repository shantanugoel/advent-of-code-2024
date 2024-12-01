use crate::utils::{self, Answer};

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let lines = utils::read_lines("./inputs/day1");
    lines
        .into_iter()
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

pub fn part1() -> Answer {
    let (mut list1, mut list2) = get_input();
    list1.sort();
    list2.sort();

    let mut distance = 0;

    for (x, y) in list1.iter().zip(list2.iter()) {
        distance += (x - y).abs();
    }

    distance.into()
}

pub fn part2() -> Answer {
    let (list1, list2) = get_input();

    let mut similarity_score = 0;
    for x in list1.iter() {
        for y in list2.iter() {
            if x == y {
                similarity_score += x;
            }
        }
    }

    similarity_score.into()
}
