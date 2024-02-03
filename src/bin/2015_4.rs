use anyhow::{Context, Result};
use tap::prelude::*;

fn main() -> Result<()> {
    let input = include_str!("../../assets/2015/04.txt");

    let silver = find_prefix(input, "00000").context("no solution for silver")?;
    let gold = find_prefix(input, "000000").context("no solution for gold")?;

    println!("silver: `{silver}`");
    println!("gold: `{gold}`");

    Ok(())
}

fn find_prefix(input: &str, prefix: &str) -> Option<u64> {
    (0..=u64::MAX).find(|v| {
        md5::compute(format!("{input}{v}"))
            .pipe(|digest| format!("{digest:x}"))
            .starts_with(prefix)
    })
}
