#![warn(clippy::pedantic)]

use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../assets/2023/09.txt");
    let parsed = parse(input);

    let (part_1, part_2) = solve(parsed.as_slice());
    println!("1 - `{part_1}`");
    println!("2 - `{part_2}`");
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| i64::from_str(v).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn solve(parsed: &[Vec<i64>]) -> (i64, i64) {
    parsed
        .iter()
        .map(|v| extrapolate(v.as_slice()))
        .reduce(|acc, v| (acc.0 + v.0, acc.1 + v.1))
        .unwrap()
}

fn extrapolate(line: &[i64]) -> (i64, i64) {
    let mut buf = vec![line.iter().copied().collect_vec()];

    loop {
        if buf.last().unwrap().iter().all(|v| *v == 0) {
            break;
        }

        let tmp = buf
            .last()
            .unwrap()
            .iter()
            .copied()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        buf.push(tmp);
    }

    buf.into_iter()
        .rev()
        .map(|v| (*v.first().unwrap(), *v.last().unwrap()))
        .reduce(|acc, (first, last)| (first - acc.0, acc.1 + last))
        .unwrap()
}
