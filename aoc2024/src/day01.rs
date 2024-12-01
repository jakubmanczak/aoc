use std::fs;

use tracing::info;

pub fn solve(input_path: &str) {
    solve_part1(input_path);
    solve_part2(input_path);
}

fn solve_part1(input_path: &str) {
    let mut total = 0;
    let (lvec, rvec) = get_num_list(input_path);
    for (l, r) in lvec.iter().zip(rvec.iter()) {
        total += (l - r).abs()
    }
    info!("total differences: {}", total);
}

fn solve_part2(input_path: &str) {
    let mut total = 0;
    let (lvec, rvec) = get_num_list(input_path);
    for l in lvec {
        let count = i32::try_from(rvec.iter().filter(|&&el| el == l).count()).unwrap();
        total += l * count;
    }
    info!("similarity score: {}", total);
}

fn get_num_list(input_path: &str) -> (Vec<i32>, Vec<i32>) {
    let content = fs::read_to_string(input_path).unwrap();
    let nums: Vec<(i32, i32)> = content
        .trim()
        .split('\n')
        .map(|str| match str.split_once(' ') {
            Some((l, r)) => (
                l.trim().parse::<i32>().unwrap(),
                r.trim().parse::<i32>().unwrap(),
            ),
            None => panic!(),
        })
        .collect();
    let mut lvec: Vec<i32> = Vec::new();
    let mut rvec: Vec<i32> = Vec::new();
    for (l, r) in nums {
        lvec.push(l);
        rvec.push(r);
    }
    lvec.sort();
    rvec.sort();

    (lvec, rvec)
}
