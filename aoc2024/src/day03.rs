use std::fs;

use tracing::info;

pub fn solve() {
    solve_part1();
    solve_part2();
}

fn solve_part1() {
    let content = fs::read_to_string("./input/03.txt").unwrap();

    let mut mul_sum = 0;
    let mut index = 0;
    while index < content.len() {
        if content[index..].starts_with("mul(") {
            index += "mul(".len();
            let numstr = content[index..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let x = numstr.parse::<i32>().unwrap();

            index += numstr.len();
            if !content[index..].starts_with(',') {
                continue;
            }
            index += ",".len();

            let numstr = content[index..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let y = numstr.parse::<i32>().unwrap();

            index += numstr.len();
            if !content[index..].starts_with(')') {
                continue;
            }

            mul_sum += x * y;
        }
        index += 1;
    }

    info!("sum of multiplications: {}", mul_sum);
}

fn solve_part2() {
    let content = fs::read_to_string("./input/03.txt").unwrap();

    let mut mul_sum = 0;
    let mut index = 0;
    let mut enabled = true;
    while index < content.len() {
        if content[index..].starts_with("do()") {
            enabled = true;
            index += "do()".len();
        }
        if content[index..].starts_with("don't()") {
            enabled = false;
            index += "don't()".len();
        }
        if enabled && content[index..].starts_with("mul(") {
            index += "mul(".len();
            let numstr = content[index..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let x = numstr.parse::<i32>().unwrap();

            index += numstr.len();
            if !content[index..].starts_with(',') {
                continue;
            }
            index += ",".len();

            let numstr = content[index..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let y = numstr.parse::<i32>().unwrap();

            index += numstr.len();
            if !content[index..].starts_with(')') {
                continue;
            }

            mul_sum += x * y;
        }
        index += 1;
    }

    info!("sum of conditional multiplications: {}", mul_sum);
}
