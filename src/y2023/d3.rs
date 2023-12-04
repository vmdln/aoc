use std::ops::Range;

use anyhow::Result;
use itertools::Itertools;

use crate::Solution;

#[derive(Default)]
pub struct Part1(Vec<(Vec<Number>, Vec<Symbol>)>);

impl Solution for Part1 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        let parsed = parse(line.as_bytes());

        self.0.push(parsed);

        Ok(())
    }

    fn finish(self) -> Result<Self::Type> {
        let mut sum = 0;

        for (a, b, c) in self.0.iter().tuple_windows() {
            for number in &b.0 {
                if has_neighbor(
                    number,
                    Some(a.1.as_slice()),
                    Some(c.1.as_slice()),
                    b.1.as_slice(),
                ) {
                    sum += number.v;
                }
            }
        }

        let first_line = &self.0[0];
        let second_line = &self.0[1];

        for number in &first_line.0 {
            if has_neighbor(
                number,
                None,
                Some(second_line.1.as_slice()),
                first_line.1.as_slice(),
            ) {
                sum += number.v;
            }
        }

        let last_line = &self.0[self.0.len() - 1];
        let second_to_last_line = &self.0[self.0.len() - 2];

        for number in &last_line.0 {
            if has_neighbor(
                number,
                None,
                Some(second_to_last_line.1.as_slice()),
                last_line.1.as_slice(),
            ) {
                sum += number.v;
            }
        }

        Ok(sum)
    }
}

#[derive(Default)]
pub struct Part2(Vec<(Vec<Number>, Vec<Symbol>)>);

impl Solution for Part2 {
    type Type = u64;

    fn process_line(&mut self, line: &str) -> Result<()> {
        let parsed = parse(line.as_bytes());

        self.0.push(parsed);

        Ok(())
    }

    fn finish(self) -> Result<Self::Type> {
        let mut sum = 0;

        for (top, current, bottom) in self.0.iter().tuple_windows() {
            for symbol in &current.1 {
                sum += has_neighbor2(
                    symbol,
                    Some(top.0.as_slice()),
                    Some(bottom.0.as_slice()),
                    current.0.as_slice(),
                );
            }
        }

        let first_line = &self.0[0];
        let second_line = &self.0[1];

        for symbol in &first_line.1 {
            sum += has_neighbor2(
                symbol,
                None,
                Some(second_line.0.as_slice()),
                first_line.0.as_slice(),
            );
        }

        let last_line = &self.0[self.0.len() - 1];
        let second_to_last_line = &self.0[self.0.len() - 2];

        for symbol in &last_line.1 {
            sum += has_neighbor2(
                symbol,
                Some(second_to_last_line.0.as_slice()),
                None,
                last_line.0.as_slice(),
            );
        }

        Ok(sum)
    }
}

fn parse(line: &[u8]) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    let mut number: Option<(u64, usize)> = None;

    for (n, a) in line.iter().enumerate() {
        if let Some((mut b, start)) = number {
            if a.is_ascii_digit() {
                b *= 10;
                b += u64::from(a - b'0');
                number = Some((b, start));
            } else {
                numbers.push(Number {
                    v: b,
                    range: start.saturating_sub(1)..n.saturating_add(1),
                });
                number = None;
                if *a != b'.' {
                    let kind = if *a == b'*' {
                        SymbolKind::Gear
                    } else {
                        SymbolKind::Other
                    };
                    symbols.push(Symbol { pos: n, kind });
                }
            }
        } else if a.is_ascii_digit() {
            number = Some((u64::from(a - b'0'), n));
        } else if *a != b'.' {
            let kind = if *a == b'*' {
                SymbolKind::Gear
            } else {
                SymbolKind::Other
            };
            symbols.push(Symbol { pos: n, kind });
        }
    }

    if let Some((b, start)) = number {
        numbers.push(Number {
            v: b,
            range: start.saturating_sub(1)..line.len().saturating_add(1),
        });
    }

    (numbers, symbols)
}

fn has_neighbor(
    number: &Number,
    top: Option<&[Symbol]>,
    bottom: Option<&[Symbol]>,
    next: &[Symbol],
) -> bool {
    if let Some(a) = top {
        for a in a {
            if number.range.contains(&a.pos) {
                return true;
            }
        }
    }
    if let Some(a) = bottom {
        for a in a {
            if number.range.contains(&a.pos) {
                return true;
            }
        }
    }
    for a in next {
        if number.range.contains(&a.pos) {
            return true;
        }
    }
    false
}

fn has_neighbor2(
    symbol: &Symbol,
    top: Option<&[Number]>,
    bottom: Option<&[Number]>,
    next: &[Number],
) -> u64 {
    let mut neighbors = 0_usize;
    let mut ratio = 1;

    if symbol.kind != SymbolKind::Gear {
        return 0;
    }

    if let Some(numbers) = top {
        for number in numbers {
            if number.range.contains(&symbol.pos) {
                neighbors += 1;
                ratio *= number.v;
            }
        }
    }
    if let Some(numbers) = bottom {
        for number in numbers {
            if number.range.contains(&symbol.pos) {
                neighbors += 1;
                ratio *= number.v;
            }
        }
    }
    for number in next {
        if number.range.contains(&symbol.pos) {
            neighbors += 1;
            ratio *= number.v;
        }
    }

    if neighbors == 2 {
        ratio
    } else {
        0
    }
}

#[derive(Debug)]
struct Number {
    v: u64,
    range: Range<usize>,
}

#[derive(Debug)]
struct Symbol {
    pos: usize,
    kind: SymbolKind,
}

#[derive(Debug, PartialEq, Eq)]
enum SymbolKind {
    Gear,
    Other,
}
