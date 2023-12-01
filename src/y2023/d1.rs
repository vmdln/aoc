use anyhow::{Context, Result};
use tap::Pipe;

use crate::Solution;

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

    pub fn finish(&self) -> Option<u8> {
        match self {
            State::None => None,
            State::One(first) => Some(first * 10 + first),
            State::Two(first, second) => Some(first * 10 + second),
        }
    }
}

#[derive(Default)]
pub struct Part1(u64);

impl Solution for Part1 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        self.0 = line
            .as_bytes()
            .iter()
            .filter(|v| v.is_ascii_digit())
            .fold(State::default(), |mut acc, v| {
                acc.push(v - b'0');
                acc
            })
            .finish()
            .context("no digits in line")?
            .pipe(|v| self.0.checked_add(v.into()))
            .context("overflow")?;

        Ok(())
    }

    fn finish(self) -> Result<u64> {
        Ok(self.0)
    }
}

#[derive(Default)]
pub struct Part2(u64);

impl Solution for Part2 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        const NEEDLES: &[(&[u8], u8)] = &[
            (b"0".as_slice(), 0),
            (b"1".as_slice(), 1),
            (b"2".as_slice(), 2),
            (b"3".as_slice(), 3),
            (b"4".as_slice(), 4),
            (b"5".as_slice(), 5),
            (b"6".as_slice(), 6),
            (b"7".as_slice(), 7),
            (b"8".as_slice(), 8),
            (b"9".as_slice(), 9),
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

        let mut line = line.as_bytes();
        let mut state = State::default();
        while !line.is_empty() {
            if let Some(v) = NEEDLES
                .iter()
                .find(|(needle, _)| line.starts_with(needle))
                .map(|(_, v)| *v)
            {
                state.push(v);
            }

            line = &line[1..];
        }

        self.0 = state
            .finish()
            .context("no digits in line")?
            .pipe(|v| self.0.checked_add(v.into()))
            .context("overflow")?;

        Ok(())
    }

    fn finish(self) -> Result<u64> {
        Ok(self.0)
    }
}
