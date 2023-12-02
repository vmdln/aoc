use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

use crate::Solution;

#[derive(Default)]
pub struct Part1(u64);

impl Solution for Part1 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        let game = dbg!(Game::from_str(line).unwrap());

        if game.set.red <= 12 && game.set.green <= 13 && game.set.blue <= 14 {
            self.0 += game.id;
        }

        Ok(())
    }

    fn finish(self) -> Result<Self::Type> {
        Ok(self.0)
    }
}

#[derive(Default)]
pub struct Part2(u64);

impl Solution for Part2 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        let game = dbg!(Game::from_str(line).unwrap());

        let power = game.set.red * game.set.green * game.set.blue;
        self.0 += power;

        Ok(())
    }

    fn finish(self) -> Result<Self::Type> {
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u64,
    pub set: Set,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(mut s: &str) -> std::result::Result<Self, Self::Err> {
        s = &s[5..];
        let (id, rest) = s.split_once(':').context("expected `:`")?;

        let set = rest
            .split([';', ','])
            .try_fold(Set::default(), |mut set, v| {
                let (v, color) = v
                    .trim_start()
                    .split_once(' ')
                    .context("cannot split color")?;
                match color {
                    "red" => {
                        set.red = set.red.max(v.parse().with_context(|| {
                            format!("unable to parse value for red, found: `{v}`")
                        })?);
                    }
                    "green" => {
                        set.green = set.green.max(v.parse().with_context(|| {
                            format!("unable to parse value for green, found: `{v}`")
                        })?);
                    }
                    "blue" => {
                        set.blue = set.blue.max(v.parse().with_context(|| {
                            format!("unable to parse value for blue, found: `{v}`")
                        })?);
                    }
                    _ => bail!("invalid color: `{color}`"),
                }

                Ok::<_, Error>(set)
            })?;

        let id = id
            .parse()
            .with_context(|| format!("unable to parse id, found: `{id}`"))?;

        Ok(Self { id, set })
    }
}

#[derive(Default, Debug)]
pub struct Set {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}
