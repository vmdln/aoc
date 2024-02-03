use std::collections::HashSet;

use tap::prelude::*;
use thiserror::Error;

fn main() -> anyhow::Result<()> {
    let input = include_bytes!("../../assets/2015/03.txt");

    let mut silver = HashSet::new();
    let mut silver_santa = Position::default();

    silver.insert(silver_santa);

    let mut gold = HashSet::new();
    let mut gold_santa = Position::default();
    let mut gold_robo = Position::default();

    gold.insert(gold_santa);

    let mut robo_turn = false;
    for v in input {
        let dir = Direction::try_from(*v)?;

        silver_santa.update(dir);
        silver.insert(silver_santa);

        if robo_turn {
            gold_robo.update(dir);
            gold.insert(gold_robo);
        } else {
            gold_santa.update(dir);
            gold.insert(gold_santa);
        }
        robo_turn = !robo_turn;
    }

    println!("silver: `{}`", silver.len());
    println!("gold: `{}`", gold.len());

    Ok(())
}

#[derive(Hash, Default, Clone, Copy, PartialEq, Eq)]
struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn update(&mut self, dir: Direction) {
        match dir {
            Direction::N => self.y += 1,
            Direction::S => self.y -= 1,
            Direction::E => self.x += 1,
            Direction::W => self.x -= 1,
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl TryFrom<u8> for Direction {
    type Error = DirectionTryFromError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let ret = match value {
            b'^' => Self::N,
            b'v' => Self::S,
            b'>' => Self::E,
            b'<' => Self::W,
            v => return DirectionTryFromError(v).pipe(Err),
        };

        Ok(ret)
    }
}

#[derive(Debug, Error)]
#[error("invalid byte: `{0}`")]
pub struct DirectionTryFromError(u8);
