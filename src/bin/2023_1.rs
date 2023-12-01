use anyhow::{Context, Error, Result};
use aoc::y2023::d1;

fn main() -> Result<()> {
    let input = include_str!("../../assets/2023/1.txt");

    let (a, b) = input.lines().try_fold((0_u64, 0_u64), |mut acc, line| {
        acc.0 = d1::part_1(line.as_bytes())
            .and_then(|v| acc.0.checked_add(v.into()).context("overflow"))?;
        acc.1 = d1::part_2(line.as_bytes())
            .and_then(|v| acc.1.checked_add(v.into()).context("overflow"))?;

        Ok::<_, Error>(acc)
    })?;

    aoc::print_results("2023/1", a, b);

    Ok(())
}
