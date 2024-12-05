use std::collections::HashSet;

use crate::utils::{self, Answer};

fn get_input() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let lines = utils::read_lines("./inputs/day5");
    let mut order: Vec<(u32, u32)> = Vec::new();
    let mut pages: Vec<Vec<u32>> = Vec::new();
    let mut process_pages = false;
    for line in lines.iter() {
        if !process_pages && line.is_empty() {
            process_pages = true;
            continue;
        }
        if process_pages {
            pages.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        } else {
            let temp = line.split_once('|').unwrap();
            order.push((temp.0.parse().unwrap(), temp.1.parse().unwrap()));
        }
    }
    (order, pages)
}

pub fn part1() -> Answer {
    let (order, pages) = get_input();
    let mut map: HashSet<String> = HashSet::new();
    let mut sum = 0;

    for page in pages.iter() {
        let mut safe = true;
        'outer: for i in 0..page.len() - 1 {
            let mut safe_segment = format!("{}", page[i]);
            for j in i + 1..page.len() {
                safe_segment.push_str(format!(",{}", page[j]).as_str());
                if map.contains(safe_segment.as_str()) {
                    continue;
                }
                for k in 0..order.len() {
                    if page[i] == order[k].1 && page[j] == order[k].0 {
                        safe = false;
                        break 'outer;
                    }
                }
                map.insert(safe_segment.clone());
            }
        }
        if safe {
            sum += page[page.len() / 2];
        }
    }
    sum.into()
}

pub fn part2() -> Answer {
    let (order, pages) = get_input();
    let mut map: HashSet<String> = HashSet::new();
    let mut sum = 0;

    for page_iter in pages.iter() {
        let mut page = page_iter.clone();
        let mut safe = true;
        for i in 0..page.len() - 1 {
            let mut safe_segment = format!("{}", page[i]);
            loop {
                let mut inner_safe = true;
                'outer: for j in i + 1..page.len() {
                    safe_segment.push_str(format!(",{}", page[j]).as_str());
                    if map.contains(safe_segment.as_str()) {
                        continue;
                    }
                    for k in 0..order.len() {
                        if page[i] == order[k].1 && page[j] == order[k].0 {
                            let temp = page[i];
                            page[i] = page[j];
                            page[j] = temp;
                            safe = false;
                            inner_safe = false;
                            break 'outer;
                        }
                    }
                    map.insert(safe_segment.clone());
                }
                if inner_safe {
                    break;
                }
            }
        }
        if !safe {
            sum += page[page.len() / 2];
        }
    }
    sum.into()
}
