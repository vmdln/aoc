use anyhow::Context;
use criterion::{black_box, Criterion};

fn solve_part_1(input: &str) {
    let v = input
        .lines()
        .try_fold(0_u64, |acc, line| {
            aoc::y2023::d1::part_1(line.as_bytes())
                .and_then(|v| acc.checked_add(v as u64).context("overflow"))
        })
        .unwrap();

    black_box(v);
}

fn solve_part_2(input: &str) {
    let v = input
        .lines()
        .try_fold(0_u64, |acc, line| {
            aoc::y2023::d1::part_2(line.as_bytes())
                .and_then(|v| acc.checked_add(v as u64).context("overflow"))
        })
        .unwrap();

    black_box(v);
}

const INPUT: &str = include_str!("../assets/2023/big_1.txt");

pub fn part_1(c: &mut Criterion) {
    c.bench_function("part 1", |b| {
        b.iter(|| solve_part_1(black_box(INPUT)));
    });
}

pub fn part_2(c: &mut Criterion) {
    c.bench_function("part 2", |b| {
        b.iter(|| solve_part_2(black_box(INPUT)));
    });
}

criterion::criterion_group!(benches, part_1, part_2);
criterion::criterion_main!(benches);
