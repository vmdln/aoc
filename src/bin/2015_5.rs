use itertools::Itertools;
use std::collections::{hash_map::Entry, HashMap};

fn main() {
    let input = include_str!("../../assets/2015/5.txt");

    let (a, b) = solve(input);
    aoc::print_results("2015/5", a, b);
}

fn solve(input: &str) -> (u64, u64) {
    let mut acc_a = 0;
    let mut acc_b = 0;

    for line in input.lines() {
        if solve_a(line.as_bytes()) {
            acc_a += 1;
        }

        if solve_b(line.as_bytes()) {
            acc_b += 1;
        }
    }

    (acc_a, acc_b)
}

fn solve_a(line: &[u8]) -> bool {
    const VOWELS: &[u8] = &[b'a', b'o', b'e', b'u', b'i'];

    let mut vowels = 0;
    let mut repeated = false;
    let mut magic_string = false;

    if let Some(a) = line.first() {
        if VOWELS.contains(a) {
            vowels += 1;
        }
    }

    for (a, b) in line.iter().tuple_windows() {
        if VOWELS.contains(b) {
            vowels += 1;
        }

        repeated |= a == b;
        magic_string |= matches!(
            (a, b),
            (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y')
        );
    }

    vowels >= 3 && repeated && !magic_string
}

fn solve_b(line: &[u8]) -> bool {
    let mut iter = line.iter().tuple_windows().peekable();

    let mut pairs = HashMap::new();
    let mut repeated = false;
    let mut interspersed = false;

    if let Some((a, b, _)) = iter.peek() {
        pairs.insert((**a, **b), 0_usize);
    }

    for ((a, b, c), n) in iter.zip(1..) {
        match pairs.entry((*b, *c)) {
            Entry::Occupied(entry) => {
                if *entry.get() < n - 1 {
                    repeated |= true;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(n);
            }
        }

        interspersed |= a == c;
    }

    repeated && interspersed
}
