use std::sync::OnceLock;

use anyhow::{Context, Result};
use checked::Checked;
use regex::Regex;
use tap::{Pipe, Tap};

fn main() -> Result<()> {
    let input = include_str!("../../assets/2015/2.txt");

    let (a, b) = solve(input)?;

    aoc::print_results("2015/2", a, b);

    Ok(())
}

fn solve(input: &str) -> Result<(u64, u64)> {
    let mut acc_a = 0;
    let mut acc_b = 0;

    for entry in input.lines() {
        let (a, b, c) = parse(entry)
            .with_context(|| format!("unable to parse `{entry}`"))
            .map(|(a, b, c)| {
                [a, b, c]
                    .tap_mut(|v| v.sort_unstable())
                    .pipe(|[a, b, c]| (Checked::new(a), Checked::new(b), Checked::new(c)))
            })?;

        acc_a += solve_a(a, b, c).with_context(|| format!("`a` overflow on `{entry}`"))?;
        acc_b += solve_b(a, b, c).with_context(|| format!("`b` overflow on `{entry}`"))?;
    }

    Ok((acc_a, acc_b))
}

fn solve_a(a: Checked<u64>, b: Checked<u64>, c: Checked<u64>) -> Option<u64> {
    let m = a * b;
    let n = a * c;
    let o = b * c;

    (m * 2 + n * 2 + o * 2 + m).0
}

fn solve_b(a: Checked<u64>, b: Checked<u64>, c: Checked<u64>) -> Option<u64> {
    let wrap = a + a + b + b;
    let ribbon = a * b * c;

    (wrap + ribbon).0
}

fn parse(s: &str) -> Option<(u64, u64, u64)> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex =
        REGEX.get_or_init(|| Regex::new("^([1-9][0-9]*)x([1-9][0-9]*)x([1-9][0-9]*)$").unwrap());

    let captures = regex.captures(s)?;

    let a: u64 = captures.get(1).unwrap().as_str().parse().ok()?;
    let b: u64 = captures.get(2).unwrap().as_str().parse().ok()?;
    let c: u64 = captures.get(3).unwrap().as_str().parse().ok()?;

    Some((a, b, c))
}
