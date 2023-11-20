use anyhow::{bail, Result};
use aoc::print_results;

fn main() -> Result<()> {
    let input = include_bytes!("../../assets/2015/1.txt");

    let a = solve_a(input)?;
    let b = solve_b(input)?;

    print_results("2015/1", a, b);

    Ok(())
}

fn solve_a(input: &[u8]) -> Result<i64> {
    let mut acc = 0;
    for v in input {
        match v {
            b'(' => acc += 1,
            b')' => acc -= 1,
            v => bail!("invalid byte encountered: {v}"),
        }
    }

    Ok(acc)
}

fn solve_b(input: &[u8]) -> Result<usize> {
    let mut acc = 0;
    for (v, n) in input.iter().zip(1..) {
        match v {
            b'(' => acc += 1,
            b')' => {
                acc -= 1;
                if acc == -1 {
                    return Ok(n);
                }
            }
            v => bail!("invalid byte encountered: {v}"),
        }
    }

    bail!("santa didn't enter the basement")
}
