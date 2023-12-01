use anyhow::{Context, Result};
use tap::prelude::*;

fn main() -> Result<()> {
    let input = include_str!("../../assets/2023/1.txt");

    let (a, b) = solve(input)?;

    aoc::print_results("2015/1", a, b);

    Ok(())
}

fn solve(input: &str) -> Result<(u64, u64)> {
    let mut acc_a = 0;
    let mut acc_b = 0;

    for line in input.lines() {
        acc_a += solve_a(line.as_bytes())?.pipe(u64::from);
        acc_b += solve_b(line.as_bytes())?.pipe(u64::from);
    }

    Ok((acc_a, acc_b))
}

fn solve_a(line: &[u8]) -> Result<u8> {
    let mut state = State::default();
    for v in line {
        if v.is_ascii_digit() {
            state.push(v - b'0')
        }
    }

    state.finalize().context("no digits in line")
}

fn solve_b(mut line: &[u8]) -> Result<u8> {
    const NEEDLES: &[(&[u8], u8)] = &[
        (b"zero".as_slice(), 0),
        (b"one".as_slice(), 1),
        (b"two".as_slice(), 2),
        (b"three".as_slice(), 3),
        (b"four".as_slice(), 4),
        (b"five".as_slice(), 5),
        (b"six".as_slice(), 6),
        (b"seven".as_slice(), 7),
        (b"eight".as_slice(), 8),
        (b"nine".as_slice(), 9),
    ];

    let mut state = State::default();
    while !line.is_empty() {
        if let Some(v) = if line[0].is_ascii_digit() {
            Some(line[0] - b'0')
        } else {
            NEEDLES
                .iter()
                .find(|(needle, _)| line.starts_with(needle))
                .map(|(_, v)| *v)
        } {
            state.push(v)
        }

        line = &line[1..];
    }

    state.finalize().context("no digits in line")
}

#[derive(Default)]
enum State {
    #[default]
    None,
    One(u8),
    Two(u8, u8),
}

impl State {
    pub fn push(&mut self, v: u8) {
        match self {
            State::None => *self = State::One(v),
            State::One(first) | State::Two(first, _) => *self = State::Two(*first, v),
        }
    }

    pub fn finalize(&self) -> Option<u8> {
        match self {
            State::None => None,
            State::One(first) => Some(first * 10 + first),
            State::Two(first, second) => Some(first * 10 + second),
        }
    }
}
