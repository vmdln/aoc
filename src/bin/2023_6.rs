use std::{fmt::Write, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};
use tap::prelude::*;

fn main() -> Result<()> {
    let input = include_str!("../../assets/2023/06.txt");
    let parsed = parse(input)?;

    let part_1 = part_1(&parsed);
    println!("1 - `{part_1}`");

    let part_2 = part_2(&parsed);
    println!("2 - `{part_2}`");

    Ok(())
}

struct Parsed {
    pub times: Vec<u64>,
    pub distances: Vec<u64>,
}

fn parse(input: &str) -> Result<Parsed> {
    let mut lines = input.lines();

    let times = lines
        .next()
        .context("missing line 1")
        .and_then(|line| parse_line(line, "Time:"))?;
    let distances = lines
        .next()
        .context("missing line 2")
        .and_then(|line| parse_line(line, "Distance:"))?;

    if lines.next().is_some() {
        bail!("trailing input")
    }

    Ok(Parsed { times, distances })
}

fn part_1(parsed: &Parsed) -> u64 {
    parsed
        .times
        .iter()
        .zip(parsed.distances.iter())
        .map(|(time, distance)| {
            (1..*time)
                .filter_map(|charge| is_beating_map(*time, *distance, charge))
                .count() as u64
        })
        .product()
}

fn part_2(parsed: &Parsed) -> u64 {
    let time = concat(&parsed.times);
    let distance = concat(&parsed.distances);

    (1..time)
        .filter_map(|charge| is_beating_map(time, distance, charge))
        .count() as u64
}

fn concat(src: &[u64]) -> u64 {
    src.iter()
        .fold(String::new(), |mut acc, v| {
            write!(&mut acc, "{v}").unwrap();
            acc
        })
        .pipe_as_ref(u64::from_str)
        .unwrap()
}

fn parse_line(line: &str, prefix: &str) -> Result<Vec<u64>> {
    line.pipe(|v| {
        v.strip_prefix(prefix)
            .ok_or_else(|| anyhow!("invalid prefix, expected `{prefix} ..`, found: `{v}`"))
    })?
    .split_ascii_whitespace()
    .map(|v| u64::from_str(v).map_err(|_| anyhow!("unable to parse: `{v}`")))
    .collect()
}

fn is_beating_map(time: u64, distance: u64, charge: u64) -> Option<u64> {
    let left = time - charge;
    let new_distance = left * charge;

    if new_distance > distance {
        Some(new_distance)
    } else {
        None
    }
}
