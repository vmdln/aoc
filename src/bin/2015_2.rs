#![warn(clippy::pedantic)]

use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use tap::prelude::*;
use winnow::{ascii::dec_uint, error::ContextError, Parser};

fn main() -> Result<()> {
    let input = include_str!("../../assets/2015/02.txt");

    let mut silver = 0;
    let mut gold = 0;
    for line in input.lines() {
        let dimensions = Dimensions::from_str(line)?;

        silver += dimensions.silver();
        gold += dimensions.gold();
    }

    println!("silver: `{silver}`");
    println!("gold: `{gold}`");

    Ok(())
}

struct Dimensions {
    l: u64,
    w: u64,
    h: u64,
}

impl Dimensions {
    pub fn silver(&self) -> u64 {
        let a = self.l * self.w;
        let b = self.l * self.h;
        let c = self.w * self.h;

        2 * a + 2 * b + 2 * c + a.min(b).min(c)
    }

    pub fn gold(&self) -> u64 {
        let [a, b, c] = [self.l, self.w, self.h].tap_mut(|v| v.sort_unstable());

        let wrap = a + a + b + b;
        let ribbon = a * b * c;

        wrap + ribbon
    }
}

impl FromStr for Dimensions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        (dec_uint::<_, _, ContextError>, 'x', dec_uint, 'x', dec_uint)
            .parse(s)
            .map(|(length, _, width, _, height)| Dimensions {
                l: length,
                w: width,
                h: height,
            })
            .map_err(|e| anyhow!("\n{e}"))
    }
}
