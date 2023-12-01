#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod y2023;

use std::fmt::Display;

use anyhow::Result;

pub fn print_results<A, B>(prefix: &str, a: A, b: B)
where
    A: Display,
    B: Display,
{
    println!("{prefix}:");
    println!("- a: {a}");
    println!("- b: {b}");
}

pub trait Solution: Default
where
    Self::Type: Display,
{
    type Type;

    fn process_line(&mut self, line: &str) -> Result<()>;
    fn finish(self) -> Result<Self::Type>;
}

#[macro_export]
macro_rules! solve {
    ($a:ty, $input:expr) => {
        |input: &str| -> anyhow::Result<<$a as aoc::Solution>::Type> {
            use aoc::Solution;

            let mut a = <$a>::default();

            for line in $input.lines() {
                a.process_line(line)?;
            }

            a.finish()
        }($input)
    };
    ($a:ty, $b:ty, $input:expr) => {
        |input: &str| -> anyhow::Result<(<$a as aoc::Solution>::Type, <$a as aoc::Solution>::Type)> {
            use aoc::Solution;

            let mut a = <$a>::default();
            let mut b = <$b>::default();

            for line in $input.lines() {
                a.process_line(line)?;
                b.process_line(line)?;
            }

            let a = a.finish()?;
            let b = b.finish()?;

            Ok((a, b))
        }($input)
    };
}

#[macro_export]
macro_rules! solve_main {
    ($path:literal, $a:ty) => {
        fn main() -> anyhow::Result<()> {
            let input = include_str!($path);
            let a_ret = aoc::solve!($a, input)?;

            println!("- `{}`", a_ret);

            Ok(())
        }
    };
    ($path:literal, $a:ty, $b:ty) => {
        fn main() -> anyhow::Result<()> {
            let input = include_str!($path);
            let (a_ret, b_ret) = aoc::solve!($a, $b, input)?;

            println!("- `{}`", a_ret);
            println!("- `{}`", b_ret);

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! bench {
    ($path:literal, $a:ty) => {
        const INPUT: &str = include_str!($path);

        pub fn part_1(c: &mut criterion::Criterion) {
            c.bench_function("part 1", |b| {
                b.iter(|| criterion::black_box(aoc::solve!($a, criterion::black_box(INPUT))));
            });
        }

        criterion::criterion_group!(benches, part_1);
        criterion::criterion_main!(benches);
    };
    ($path:literal, $a:ty, $b:ty) => {
        const INPUT: &str = include_str!($path);

        pub fn part_1(c: &mut criterion::Criterion) {
            c.bench_function("part 1", |b| {
                b.iter(|| {
                    criterion::black_box(aoc::solve!($a, criterion::black_box(INPUT)).unwrap())
                });
            });
        }

        pub fn part_2(c: &mut criterion::Criterion) {
            c.bench_function("part 2", |b| {
                b.iter(|| {
                    criterion::black_box(aoc::solve!($b, criterion::black_box(INPUT)).unwrap())
                });
            });
        }

        criterion::criterion_group!(benches, part_1, part_2);
        criterion::criterion_main!(benches);
    };
}
