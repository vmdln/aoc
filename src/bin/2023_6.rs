use anyhow::Result;
use aoc::y2023::d6::{self, Parser};

fn main() -> Result<()> {
    let input = include_str!("../../assets/2023/06.txt");

    let parsed = Parser::parse(input)?;
    dbg!(d6::part1(&parsed));
    dbg!(d6::part2(&parsed));

    Ok(())
}
