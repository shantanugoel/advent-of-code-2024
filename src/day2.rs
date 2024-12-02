use crate::utils::{self, Answer};

fn get_input() -> Vec<Vec<i32>> {
    let lines = utils::read_lines("./inputs/day2");
    lines
        .into_iter()
        .map(|s| s.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

enum ReportState {
    Unknown,
    Descending,
    Ascending,
}

pub fn part1() -> Answer {
    let reports = get_input();
    let total = reports.len();
    let mut unsafe_count = 0;
    for report in reports.iter() {
        let mut report_state = ReportState::Unknown;
        for window in report.windows(2) {
            match report_state {
                ReportState::Unknown => {
                    if window[0] > window[1] {
                        report_state = ReportState::Descending;
                    } else if window[0] < window[1] {
                        report_state = ReportState::Ascending;
                    } else {
                        unsafe_count += 1;
                        break;
                    }
                }
                ReportState::Descending => {
                    if window[0] < window[1] {
                        unsafe_count += 1;
                        break;
                    }
                }
                ReportState::Ascending => {
                    if window[0] > window[1] {
                        unsafe_count += 1;
                        break;
                    }
                }
            }
            let diff = (window[0] - window[1]).abs();
            if diff > 3 || diff == 0 {
                unsafe_count += 1;
                break;
            }
        }
    }
    (total - unsafe_count).into()
}

fn check_safe(report: &Vec<i32>) -> bool {
    let mut report_state = ReportState::Unknown;
    let mut safe = true;
    for window in report.windows(2) {
        match report_state {
            ReportState::Unknown => {
                if window[0] > window[1] {
                    report_state = ReportState::Descending;
                } else if window[0] < window[1] {
                    report_state = ReportState::Ascending;
                } else {
                    safe = false;
                    break;
                }
            }
            ReportState::Descending => {
                if window[0] <= window[1] {
                    safe = false;
                    break;
                }
            }
            ReportState::Ascending => {
                if window[0] >= window[1] {
                    safe = false;
                    break;
                }
            }
        }
        if (window[0] - window[1]).abs() > 3 {
            safe = false;
            break;
        }
    }
    safe
}

pub fn part2() -> Answer {
    let mut reports = get_input();
    let total = reports.len();
    let mut unsafe_count = 0;

    for report in reports.iter_mut() {
        let mut is_safe = check_safe(report);
        if !is_safe {
            for i in 0..report.len() {
                let mut report_clone = report.clone();
                _ = report_clone.remove(i);
                is_safe = check_safe(&report_clone);
                if is_safe {
                    break;
                }
            }
        }
        if !is_safe {
            unsafe_count += 1;
        } else {
        }
    }
    (total - unsafe_count).into()
}
