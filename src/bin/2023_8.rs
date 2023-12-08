use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::OnceLock,
};

use itertools::Itertools;
use tap::prelude::*;

fn main() {
    let input = include_str!("../../assets/2023/08.txt");
    let parsed = parse(input);

    let part_1 = part_1(&parsed);
    println!("1 - `{part_1}`");

    let part_2 = part_2(&parsed);
    println!("2 - `{part_2}`");
}

#[derive(Debug)]
pub struct Parsed<'a> {
    pub instructions: Vec<Instruction>,
    pub map: HashMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|v| match v {
            b'R' => Instruction::Right,
            b'L' => Instruction::Left,
            _ => panic!(),
        })
        .collect_vec();

    lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines {
        let (key, rest) = line.split_once(" = (").unwrap();
        let (left, rest) = rest.split_once(", ").unwrap();
        let right = rest.strip_suffix(')').unwrap();

        map.insert(key, (left, right));
    }

    Parsed { instructions, map }
}

fn part_1(parsed: &Parsed) -> u64 {
    let mut current = parsed
        .map
        .get_key_value("AAA")
        .unwrap()
        .pipe(|(a, b)| (*a, *b));

    for (instruction, i) in parsed.instructions.iter().cycle().zip(0..) {
        if current.0 == "ZZZ" {
            return i;
        }

        match instruction {
            Instruction::Left => {
                current = get_left(&current, &parsed.map);
            }
            Instruction::Right => {
                current = get_right(&current, &parsed.map);
            }
        }
    }

    0
}

fn part_2(parsed: &Parsed) -> u64 {
    parsed
        .map
        .keys()
        .copied()
        .filter(|v| v.ends_with('A'))
        .fold(1_u64, |acc, key| {
            let mut current = parsed
                .map
                .get_key_value(key)
                .unwrap()
                .pipe(|(a, b)| (*a, *b));

            for (instruction, i) in parsed.instructions.iter().cycle().zip(1..) {
                if current.0.ends_with('Z') {
                    return num::integer::lcm(acc, i);
                }

                match instruction {
                    Instruction::Left => {
                        current = get_left(&current, &parsed.map);
                    }
                    Instruction::Right => {
                        current = get_right(&current, &parsed.map);
                    }
                }
            }

            0
        })
}

fn get_left<'a, 'b, 'c>(
    current: &(&str, (&str, &str)),
    map: &HashMap<&'a str, (&'b str, &'c str)>,
) -> (&'a str, (&'b str, &'c str)) {
    map.get_key_value(current.1 .0)
        .unwrap()
        .pipe(|(a, b)| (*a, *b))
}

fn get_right<'a, 'b, 'c>(
    current: &(&str, (&str, &str)),
    map: &HashMap<&'a str, (&'b str, &'c str)>,
) -> (&'a str, (&'b str, &'c str)) {
    map.get_key_value(current.1 .1)
        .unwrap()
        .pipe(|(a, b)| (*a, *b))
}
