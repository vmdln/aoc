#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod y2023;

use std::{fmt::Display, path::Path};

use anyhow::Result;
use getset::CopyGetters;
use image::{ImageResult, Rgb, RgbImage};
use itertools::Itertools;
use nalgebra::Vector2;
use tap::prelude::*;
use thiserror::Error;

//  #[macro_use]
//  extern crate scan_fmt;

//  pub fn print_results<A, B>(prefix: &str, a: A, b: B)
//  where
//      A: Display,
//      B: Display,
//  {
//      println!("{prefix}:");
//      println!("- a: {a}");
//      println!("- b: {b}");
//  }

//  pub trait Solution: Default
//  where
//      Self::Type: Display,
//  {
//      type Type;

//      fn process_line(&mut self, line: &str) -> Result<()>;
//      fn finish(self) -> Result<Self::Type>;
//  }

//  #[macro_export]
//  macro_rules! solve {
//      ($a:ty, $input:expr) => {
//          |input: &str| -> anyhow::Result<<$a as aoc::Solution>::Type> {
//              use aoc::Solution;

//              let mut a = <$a>::default();

//              for line in $input.lines() {
//                  a.process_line(line)?;
//              }

//              a.finish()
//          }($input)
//      };
//      ($a:ty, $b:ty, $input:expr) => {
//          |input: &str| -> anyhow::Result<(<$a as aoc::Solution>::Type, <$a as aoc::Solution>::Type)> {
//              use aoc::Solution;

//              let mut a = <$a>::default();
//              let mut b = <$b>::default();

//              for line in $input.lines() {
//                  a.process_line(line)?;
//                  b.process_line(line)?;
//              }

//              let a = a.finish()?;
//              let b = b.finish()?;

//              Ok((a, b))
//          }($input)
//      };
//  }

//  #[macro_export]
//  macro_rules! solve_main {
//      ($path:literal, $a:ty) => {
//          fn main() -> anyhow::Result<()> {
//              let input = include_str!($path);
//              let a_ret = aoc::solve!($a, input)?;

//              println!("- `{}`", a_ret);

//              Ok(())
//          }
//      };
//      ($path:literal, $a:ty, $b:ty) => {
//          fn main() -> anyhow::Result<()> {
//              let input = include_str!($path);
//              let (a_ret, b_ret) = aoc::solve!($a, $b, input)?;

//              println!("- `{}`", a_ret);
//              println!("- `{}`", b_ret);

//              Ok(())
//          }
//      };
//  }

//  #[macro_export]
//  macro_rules! bench {
//      ($path:literal, $a:ty) => {
//          const INPUT: &str = include_str!($path);

//          pub fn part_1(c: &mut criterion::Criterion) {
//              c.bench_function("part 1", |b| {
//                  b.iter(|| criterion::black_box(aoc::solve!($a, criterion::black_box(INPUT))));
//              });
//          }

//          criterion::criterion_group!(benches, part_1);
//          criterion::criterion_main!(benches);
//      };
//      ($path:literal, $a:ty, $b:ty) => {
//          const INPUT: &str = include_str!($path);

//          pub fn part_1(c: &mut criterion::Criterion) {
//              c.bench_function("part 1", |b| {
//                  b.iter(|| {
//                      criterion::black_box(aoc::solve!($a, criterion::black_box(INPUT)).unwrap())
//                  });
//              });
//          }

//          pub fn part_2(c: &mut criterion::Criterion) {
//              c.bench_function("part 2", |b| {
//                  b.iter(|| {
//                      criterion::black_box(aoc::solve!($b, criterion::black_box(INPUT)).unwrap())
//                  });
//              });
//          }

//          criterion::criterion_group!(benches, part_1, part_2);
//          criterion::criterion_main!(benches);
//      };
//  }

pub trait Parser {
    type Type;

    fn parse(input: &str) -> Result<Self::Type>;
}

pub trait Solution
where
    Self::Ret: Display,
{
    type Parsed;
    type Ret;

    fn solve(parsed: &Self::Parsed) -> Result<Self::Ret>;
}

#[macro_export]
macro_rules! aoc {
    ($parser:ty, $path:literal, $part_1:ty) => {
        fn main() -> anyhow::Result<()> {
            let input = include_str!($path);

            let parsed = parser::parse(input)?;

            let ret_1 = part_1::solve(&parsed)?;
            println!("- `{ret_1}`");

            Ok(())
        }
    };
    ($parser:ty, $path:literal, $part_1:ty, $part_2:ty) => {
        fn main() -> anyhow::Result<()> {
            use aoc::{Parser, Solution};

            let input = include_str!($path);

            let parsed = <$parser>::parse(input)?;

            let ret_1 = <$part_1>::solve(&parsed)?;
            println!("- `{ret_1}`");

            let ret_2 = <$part_2>::solve(&parsed)?;
            println!("- `{ret_2}`");

            Ok(())
        }
    };
}

#[derive(CopyGetters)]
pub struct Grid<T> {
    #[getset(get_copy = "pub")]
    size: Vector2<usize>,
    data: Box<[T]>,
}

impl<T> Grid<T> {
    #[must_use]
    pub fn new_default(size: Vector2<usize>) -> Option<Self>
    where
        T: Default,
    {
        size.y
            .checked_mul(size.x)
            .and_then(|v| {
                if isize::try_from(v).is_ok() {
                    Some(v)
                } else {
                    None
                }
            })
            .map(|len| {
                let data = (0..len)
                    .map(|_| T::default())
                    .collect_vec()
                    .into_boxed_slice();
                Self { size, data }
            })
    }

    #[must_use]
    pub fn data(&self) -> &[T] {
        self.data.as_ref()
    }

    #[must_use]
    pub fn get(&self, pos: Vector2<usize>) -> Option<&T> {
        pos.y
            .checked_mul(self.size.x)
            .and_then(|v| v.checked_add(pos.x))
            .and_then(|v| self.data.get(v))
    }

    #[must_use]
    pub fn get_mut(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        pos.y
            .checked_mul(self.size.x)
            .and_then(|v| v.checked_add(pos.x))
            .and_then(|v| self.data.get_mut(v))
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks_exact(self.size.x)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_exact_mut(self.size.x)
    }

    #[must_use]
    pub fn get_up(&self, pos: Vector2<usize>) -> Option<&T> {
        pos.y
            .checked_sub(1)
            .and_then(|y| self.get(Vector2::new(pos.x, y)))
    }

    #[must_use]
    pub fn get_mut_up(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        pos.y
            .checked_sub(1)
            .and_then(|y| self.get_mut(Vector2::new(pos.x, y)))
    }

    #[must_use]
    pub fn get_down(&self, pos: Vector2<usize>) -> Option<&T> {
        pos.y
            .checked_add(1)
            .and_then(|y| self.get(Vector2::new(pos.x, y)))
    }

    #[must_use]
    pub fn get_mut_down(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        pos.y
            .checked_add(1)
            .and_then(|y| self.get_mut(Vector2::new(pos.x, y)))
    }

    #[must_use]
    pub fn get_left(&self, pos: Vector2<usize>) -> Option<&T> {
        pos.x
            .checked_sub(1)
            .and_then(|x| self.get(Vector2::new(x, pos.y)))
    }

    #[must_use]
    pub fn get_mut_left(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        pos.x
            .checked_sub(1)
            .and_then(|x| self.get_mut(Vector2::new(x, pos.y)))
    }

    #[must_use]
    pub fn get_right(&self, pos: Vector2<usize>) -> Option<&T> {
        pos.x
            .checked_add(1)
            .and_then(|x| self.get(Vector2::new(x, pos.y)))
    }

    #[must_use]
    pub fn get_mut_right(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        pos.x
            .checked_add(1)
            .and_then(|x| self.get_mut(Vector2::new(x, pos.y)))
    }

    #[must_use]
    pub fn get_up_right(&self, pos: Vector2<usize>) -> Option<&T> {
        let x = pos.x.checked_add(1)?;
        let y = pos.y.checked_sub(1)?;

        self.get(Vector2::new(x, y))
    }

    #[must_use]
    pub fn get_mut_up_right(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        let x = pos.x.checked_add(1)?;
        let y = pos.y.checked_sub(1)?;

        self.get_mut(Vector2::new(x, y))
    }

    #[must_use]
    pub fn get_down_right(&self, pos: Vector2<usize>) -> Option<&T> {
        let x = pos.x.checked_add(1)?;
        let y = pos.y.checked_add(1)?;

        self.get(Vector2::new(x, y))
    }

    #[must_use]
    pub fn get_mut_down_right(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        let x = pos.x.checked_add(1)?;
        let y = pos.y.checked_add(1)?;

        self.get_mut(Vector2::new(x, y))
    }

    #[must_use]
    pub fn get_up_left(&self, pos: Vector2<usize>) -> Option<&T> {
        let x = pos.x.checked_sub(1)?;
        let y = pos.y.checked_sub(1)?;

        self.get(Vector2::new(x, y))
    }

    #[must_use]
    pub fn get_mut_up_left(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        let x = pos.x.checked_sub(1)?;
        let y = pos.y.checked_sub(1)?;

        self.get_mut(Vector2::new(x, y))
    }

    #[must_use]
    pub fn get_down_left(&self, pos: Vector2<usize>) -> Option<&T> {
        let x = pos.x.checked_sub(1)?;
        let y = pos.y.checked_add(1)?;

        self.get(Vector2::new(x, y))
    }

    #[must_use]
    pub fn get_mut_down_left(&mut self, pos: Vector2<usize>) -> Option<&mut T> {
        let x = pos.x.checked_sub(1)?;
        let y = pos.y.checked_add(1)?;

        self.get_mut(Vector2::new(x, y))
    }

    pub fn save_image<'a, P>(&'a self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
        &'a T: Into<Rgb<u8>>,
    {
        let mut image = RgbImage::new(self.size.x as u32, self.size.y as u32);
        for (y, row) in self.rows().enumerate() {
            for (x, v) in row.iter().enumerate() {
                image.put_pixel(x as u32, y as u32, v.into());
            }
        }

        image.save(path)
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = GridTryFromError;

    fn try_from(value: Vec<Vec<T>>) -> std::prelude::v1::Result<Self, Self::Error> {
        let height = value.len();
        let width = value.get(0).map_or(0, Vec::len);

        let mut acc = height
            .checked_mul(width)
            .and_then(|v| {
                if isize::try_from(v).is_ok() {
                    Some(v)
                } else {
                    None
                }
            })
            .map(Vec::with_capacity)
            .ok_or(GridTryFromError::TooManyElements)?;

        for mut row in value {
            if row.len() != width {
                return Err(GridTryFromError::InequalRows);
            }
            acc.append(&mut row);
        }

        Self {
            size: Vector2::new(width, height),
            data: acc.into_boxed_slice(),
        }
        .pipe(Ok)
    }
}

#[derive(Error, Debug)]
pub enum GridTryFromError {
    #[error("too many elements")]
    TooManyElements,
    #[error("rows of inequal length encountered")]
    InequalRows,
}
