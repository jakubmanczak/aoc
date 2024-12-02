use std::fs;
use tracing::info;

pub fn solve() {
    solve_part1("./input/02.txt");
    solve_part2("./input/02.txt");
}

fn solve_part1(path: &str) {
    let reports = get_reports(path);
    let mut safe_reports = 0;
    for report in reports {
        let increasing = report.windows(2).all(|w| w[0] < w[1]);
        let decreasing = report.windows(2).all(|w| w[0] > w[1]);
        let gentle_change = report.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);
        if (increasing || decreasing) && gentle_change {
            safe_reports += 1;
        }
    }
    info!("safe reports: {}", safe_reports);
}

fn solve_part2(path: &str) {
    let reports = get_reports(path);
    let mut safe_reports = 0;
    for report in reports {
        let mut variants: Vec<Vec<i32>> = vec![report.clone()];
        for i in 0..report.len() {
            let vec: Vec<i32> = report
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(index, _)| *index != i)
                .map(|(_, v)| v)
                .collect();
            variants.push(vec);
        }
        let safe = variants.iter().any(|vec| {
            let increasing = vec.windows(2).all(|w| w[0] < w[1]);
            let decreasing = vec.windows(2).all(|w| w[0] > w[1]);
            let gentle_change = vec.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);
            return (increasing || decreasing) && gentle_change;
        });
        if safe {
            safe_reports += 1;
        }
    }
    info!("safe reports w/ dampening: {}", safe_reports);
}

fn get_reports(path: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(path).unwrap();
    let reports: Vec<Vec<i32>> = content
        .trim()
        .split('\n')
        .map(|report| {
            report
                .split(' ')
                .map(|str| str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    reports
}
