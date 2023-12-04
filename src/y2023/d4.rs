use std::{collections::HashSet, ops::Range, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

use crate::Solution;

#[derive(Default)]
pub struct Part1(u64);

impl Solution for Part1 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        let card = Card::from_str(line).unwrap();

        let winning = card
            .have
            .iter()
            .filter(|v| card.winning.contains(v))
            .count();

        self.0 += if winning > 0 {
            2_u64.pow(winning as u32 - 1)
        } else {
            0
        };

        Ok(())
    }

    fn finish(self) -> Result<Self::Type> {
        Ok(self.0)
    }
}

#[derive(Default)]
pub struct Part2(Vec<Card>);

impl Solution for Part2 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        let card = Card::from_str(line).unwrap();
        self.0.push(card);

        Ok(())
    }

    fn finish(mut self) -> Result<Self::Type> {
        let mut acc = Vec::new();
        for card in self.0 {
            acc.push((card, 1_u64));
        }

        let mut ret = 0;
        for card in 0..acc.len() {
            let winning = acc[card]
                .0
                .have
                .iter()
                .filter(|v| acc[card].0.winning.contains(v))
                .count();

            for won in (card + 1..card + 1 + winning) {
                acc[won].1 += acc[card].1;
            }
            ret += acc[card].1;
        }

        Ok(ret)
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: u64,
    pub winning: HashSet<u64>,
    pub have: HashSet<u64>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let s = &s[4..].trim_start();

        let (id, s) = s.split_once(':').unwrap();
        let id = id.parse().unwrap();

        let mut winning = HashSet::new();
        let mut have = HashSet::new();

        #[derive(PartialEq, Eq)]
        enum State {
            Winning,
            Have,
        }
        let mut state = State::Winning;
        for entry in s.split_ascii_whitespace() {
            if entry == "|" {
                assert!(state == State::Winning);
                state = State::Have;

                continue;
            }

            match state {
                State::Winning => {
                    let entry = entry.parse().unwrap();
                    winning.insert(entry);
                }
                State::Have => {
                    let entry = entry.parse().unwrap();
                    have.insert(entry);
                }
            }
        }

        Ok(Self { id, winning, have })
    }
}
