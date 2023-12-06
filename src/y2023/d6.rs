use std::{fmt::Write, str::FromStr};

use anyhow::{anyhow, bail, Result};
use tap::prelude::*;

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<Parsed> {
        let mut lines = input.lines();

        let times = lines
            .next()
            .unwrap()
            .trim_start_matches("Time:")
            .split_ascii_whitespace()
            .map(|v| u64::from_str(v).map_err(|_| anyhow!("unable to parse: `{v}`")))
            .collect::<Result<Vec<u64>>>()?;
        let distances = lines
            .next()
            .unwrap()
            .trim_start_matches("Distance:")
            .split_ascii_whitespace()
            .map(|v| u64::from_str(v).map_err(|_| anyhow!("unable to parse: `{v}`")))
            .collect::<Result<Vec<u64>>>()?;

        if lines.next().is_some() {
            bail!("trailing input")
        }

        Ok(Parsed { times, distances })
    }
}

#[derive(Debug)]
pub struct Parsed {
    pub times: Vec<u64>,
    pub distances: Vec<u64>,
}

pub fn part1(parsed: &Parsed) -> u64 {
    parsed
        .times
        .iter()
        .zip(parsed.distances.iter())
        .map(|(time, distance)| {
            (1..*time)
                .filter_map(|charge| is_beating(*time, *distance, charge))
                .count() as u64
        })
        .product()
}

pub fn part2(parsed: &Parsed) -> u64 {
    let time = parsed
        .times
        .iter()
        .fold(String::new(), |mut acc, v| {
            write!(&mut acc, "{v}").unwrap();
            acc
        })
        .pipe_as_ref(u64::from_str)
        .unwrap();
    let distance = parsed
        .distances
        .iter()
        .fold(String::new(), |mut acc, v| {
            write!(&mut acc, "{v}").unwrap();
            acc
        })
        .pipe_as_ref(u64::from_str)
        .unwrap();

    (1..time)
        .filter_map(|charge| is_beating(time, distance, charge))
        .count() as u64
}

#[must_use]
pub fn is_beating(time: u64, distance: u64, charge: u64) -> Option<u64> {
    let left = time - charge;
    let new_distance = left * charge;

    if new_distance > distance {
        Some(new_distance)
    } else {
        None
    }
}
