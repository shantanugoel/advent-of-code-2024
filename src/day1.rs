use crate::utils;

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let lines = utils::read_lines("./inputs/day1");
    lines.into_iter()
    .map(|s| {
        let parts: Vec<&str> = s.split_whitespace().collect();
        (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
    }).unzip()
}

pub fn part1() {
    let mut list1: Vec<i32>;
    let mut list2: Vec<i32>;
    (list1, list2) = get_input();
    list1.sort();
    list2.sort();

    let mut distance = 0;

    for (x, y) in list1.iter().zip(list2.iter()) {
        distance += (x - y).abs();
    }

    println!("Part 1: {}", distance);
}