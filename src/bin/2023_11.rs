#![warn(clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("../../assets/2023/11.txt");
    let parsed = parse(input);

    let (part_1, part_2) = solve(&parsed);

    println!("1 - `{part_1}`");
    println!("2 - `{part_2}`");
}

fn parse(input: &str) -> Vec<Star> {
    enum State {
        FirstLine,
        Rest { rows: u64, columns: u64 },
    }

    let (state, mut stars) = input.lines().enumerate().fold(
        (State::FirstLine, HashMap::new()),
        |(mut state, mut stars), (y, line)| match &mut state {
            State::FirstLine => {
                let mut columns = 0_u64;
                for ((x, cell), column) in line.as_bytes().iter().copied().enumerate().zip(1..) {
                    columns = column;
                    if cell == b'#' {
                        stars.insert((x as u64, y as u64), (0_u64, 0_u64));
                    }
                }

                state = State::Rest { rows: 1, columns };
                (state, stars)
            }
            State::Rest { rows, columns } => {
                let mut new_columns = 0_u64;
                for ((x, cell), column) in line.as_bytes().iter().copied().enumerate().zip(1..) {
                    new_columns = column;
                    if cell == b'#' {
                        stars.insert((x as u64, y as u64), (0, 0));
                    }
                }
                assert_eq!(new_columns, *columns);
                *rows += 1;

                (state, stars)
            }
        },
    );

    let (rows, columns) = match state {
        State::FirstLine => panic!(),
        State::Rest { rows, columns } => (rows, columns),
    };

    let mut shift = 0;
    for row in 0..rows {
        let mut any_in_row = false;
        for ((_, y), (_, shift_y)) in &mut stars {
            if *y == row {
                any_in_row = true;
            }
            if *y >= row {
                *shift_y = shift;
            }
        }
        if !any_in_row {
            shift += 1;
        }
    }

    let mut shift = 0;
    for column in 0..columns {
        let mut any_in_column = false;
        for ((x, _), (shift_x, _)) in &mut stars {
            if *x == column {
                any_in_column = true;
            }
            if *x >= column {
                *shift_x = shift;
            }
        }
        if !any_in_column {
            shift += 1;
        }
    }

    stars
        .into_iter()
        .map(|(pos, offset)| Star { pos, offset })
        .collect()
}

fn solve(parsed: &[Star]) -> (u64, u64) {
    let mut part_1 = 0;
    let mut part_2 = 0;
    for (a, i) in (0..parsed.len()).zip(1..) {
        for b in i..parsed.len() {
            let a = &parsed[a];
            let b = &parsed[b];

            part_1 += a.distance(b, 1);
            part_2 += a.distance(b, 999_999);
        }
    }

    (part_1, part_2)
}

pub struct Star {
    pos: (u64, u64),
    offset: (u64, u64),
}

impl Star {
    pub fn get(&self, mul: u64) -> (u64, u64) {
        let a = self.pos.0 + self.offset.0 * mul;
        let b = self.pos.1 + self.offset.1 * mul;

        (a, b)
    }

    pub fn distance(&self, other: &Self, mul: u64) -> u64 {
        let (x_0, y_0) = self.get(mul);
        let (x_1, y_1) = other.get(mul);

        let x = if x_0 >= x_1 { x_0 - x_1 } else { x_1 - x_0 };
        let y = if y_0 >= y_1 { y_0 - y_1 } else { y_1 - y_0 };

        x + y
    }
}
