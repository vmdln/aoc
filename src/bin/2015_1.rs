use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let input = include_bytes!("../../assets/2015/01.txt");

    let mut silver = 0_i64;
    let mut gold = None;

    for (v, i) in input.iter().zip(1..) {
        match v {
            b'(' => silver += 1,
            b')' => silver -= 1,
            v => bail!("encountered invalid byte: `{v}`"),
        }

        if silver == -1 && gold.is_none() {
            gold = Some(i);
        }
    }
    let gold = gold.context("no solution for gold found")?;

    println!("silver: `{silver}`");
    println!("gold: `{gold}`");

    Ok(())
}
