use std::collections::HashSet;

use anyhow::{Context, Result};
use thiserror::Error;

fn main() -> Result<()> {
    let input = include_bytes!("../../assets/2015/3.txt");

    let (a, b) = solve(input)?;

    aoc::print_results("2015/3", a, b);

    Ok(())
}

fn solve(input: &[u8]) -> Result<(usize, usize)> {
    let mut a_acc = HashSet::new();
    a_acc.insert((0, 0));

    let mut b_acc = HashSet::new();
    b_acc.insert((0, 0));

    let mut a = Position::default();
    let mut b_santa = Position::default();
    let mut b_robo = Position::default();

    let mut turn = true;

    for (i, v) in input.iter().enumerate() {
        let direction = Direction::try_from(v)?;

        a.apply(&direction)
            .with_context(|| format!("overflow at pos `{i}`"))?;
        a_acc.insert(a.get());

        if turn {
            b_santa
                .apply(&direction)
                .with_context(|| format!("overflow at pos `{i}`"))?;
            b_acc.insert(b_santa.get());
        } else {
            b_robo
                .apply(&direction)
                .with_context(|| format!("overflow at pos `{i}`"))?;
            b_acc.insert(b_robo.get());
        }
        turn = !turn;
    }

    Ok((a_acc.len(), b_acc.len()))
}

#[derive(Default)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn right(&mut self) -> Option<()> {
        self.x = self.x.checked_add(1)?;
        Some(())
    }

    pub fn left(&mut self) -> Option<()> {
        self.x = self.x.checked_sub(1)?;
        Some(())
    }

    pub fn up(&mut self) -> Option<()> {
        self.y = self.y.checked_add(1)?;
        Some(())
    }

    pub fn down(&mut self) -> Option<()> {
        self.y = self.y.checked_sub(1)?;
        Some(())
    }

    pub fn apply(&mut self, direction: &Direction) -> Option<()> {
        match direction {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            Direction::Up => self.up(),
        }
    }

    pub fn get(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl TryFrom<&u8> for Direction {
    type Error = DirectionTryFromError;
    fn try_from(value: &u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            b'<' => Self::Left,
            b'>' => Self::Right,
            b'v' => Self::Down,
            b'^' => Self::Up,
            v => return Err(DirectionTryFromError(*v)),
        })
    }
}

#[derive(Error, Debug)]
#[error("invalid byte: `{0}`")]
struct DirectionTryFromError(u8);
