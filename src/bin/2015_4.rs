use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = include_str!("../../assets/2015/4.txt");

    let (a, b) = solve(input)?;
    aoc::print_results("2015/3", a, b);

    Ok(())
}

fn solve(input: &str) -> Result<(u64, u64)> {
    let a = solve_a(input)?;
    let b = solve_b(input, a)?;

    Ok((a, b))
}

fn solve_a(input: &str) -> Result<u64> {
    for i in 0..=u64::MAX {
        let digest = md5::compute(&format!("{input}{i}"));
        if format!("{digest:x}").starts_with("00000") {
            return Ok(i);
        }
    }

    bail!("no solution for `a` exists")
}
fn solve_b(input: &str, start: u64) -> Result<u64> {
    for i in start..=u64::MAX {
        let digest = md5::compute(&format!("{input}{i}"));
        if format!("{digest:x}").starts_with("000000") {
            return Ok(i);
        }
    }

    bail!("no solution for `b` exists")
}
