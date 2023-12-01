use anyhow::{Context, Result};

pub fn part_1(line: &[u8]) -> Result<u8> {
    line.iter()
        .filter(|v| v.is_ascii_digit())
        .fold(State::default(), |mut acc, v| {
            acc.push(v - b'0');
            acc
        })
        .finalize()
        .context("no digits in line")
}

pub fn part_2(mut line: &[u8]) -> Result<u8> {
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

    state.finalize().context("no digits in line")
}

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

    pub fn finalize(&self) -> Option<u8> {
        match self {
            State::None => None,
            State::One(first) => Some(first * 10 + first),
            State::Two(first, second) => Some(first * 10 + second),
        }
    }
}
