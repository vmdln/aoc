use std::{fmt::Write, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};
use tap::prelude::*;

#[derive(Default)]
pub struct Parser {
    state: State,
    times: Option<Vec<u64>>,
    distances: Option<Vec<u64>>,
}

#[derive(Default)]
pub enum State {
    #[default]
    Time,
    Distance,
    Done,
}

impl Parser {
    pub fn update(&mut self, line: &str) -> Result<()> {
        match self.state {
            State::Time => {
                let input = line.trim_start_matches("Time:");
                self.times = input
                    .split_ascii_whitespace()
                    .map(|v| u64::from_str(v).map_err(|_| anyhow!("unable to parse: `{v}`")))
                    .collect::<Result<Vec<u64>>>()?
                    .pipe(Some);
                self.state = State::Distance;
            }
            State::Distance => {
                let input = line.trim_start_matches("Distance:");
                self.distances = input
                    .split_ascii_whitespace()
                    .map(|v| u64::from_str(v).map_err(|_| anyhow!("unable to parse: `{v}`")))
                    .collect::<Result<Vec<u64>>>()?
                    .pipe(Some);
                self.state = State::Done;
            }
            State::Done => bail!("aoeu"),
        }

        Ok(())
    }

    pub fn finish(self) -> Result<Parsed> {
        let times = self.times.context("missing times")?;
        let distances = self.distances.context("missing distances")?;

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

pub fn is_beating(time: u64, distance: u64, charge: u64) -> Option<u64> {
    let left = time - charge;
    let new_distance = left * charge;

    if new_distance > distance {
        Some(new_distance)
    } else {
        None
    }
}
