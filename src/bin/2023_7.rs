use std::cmp::Ordering;

use itertools::Itertools;
use tap::prelude::*;

fn main() {
    let input = include_str!("../../assets/2023/07.txt");
    let parsed = parse(input);

    let part_1 = part_1(&parsed);
    println!("1 - `{part_1}`");

    let part_2 = part_2(&parsed);
    println!("2 - `{part_2}`");
}

#[derive(Clone)]
pub struct Hand {
    pub cards: [u8; 5],
    pub bid: u64,
}

fn parse(input: &str) -> Vec<Hand> {
    let mut ret = Vec::new();

    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();

        let cards = cards.as_bytes().to_vec().try_into().unwrap();
        let bid = bid.parse().unwrap();

        ret.push(Hand { cards, bid })
    }

    ret
}

fn part_1(parsed: &[Hand]) -> u64 {
    let mut tmp = parsed.to_vec();
    tmp.sort_unstable_by(|Hand { cards: a, .. }, Hand { cards: b, .. }| cmp_1(a, b));

    tmp.into_iter()
        .zip(1..)
        .map(|(Hand { bid, .. }, rank)| bid * rank)
        .sum()
}

fn part_2(parsed: &[Hand]) -> u64 {
    let mut tmp = parsed.to_vec();
    tmp.sort_unstable_by(|Hand { cards: a, .. }, Hand { cards: b, .. }| cmp_2(a, b));

    tmp.into_iter()
        .zip(1..)
        .map(|(Hand { bid, .. }, rank)| bid * rank)
        .sum()
}

fn cmp_1(a: &[u8; 5], b: &[u8; 5]) -> Ordering {
    const VALUES: &[(u8, u8)] = &[
        (b'2', 2),
        (b'3', 3),
        (b'4', 4),
        (b'5', 5),
        (b'6', 6),
        (b'7', 7),
        (b'8', 8),
        (b'9', 9),
        (b'T', 10),
        (b'J', 11),
        (b'Q', 12),
        (b'K', 13),
        (b'A', 14),
    ];

    match order_1(a).cmp(&order_1(b)) {
        Ordering::Equal => {
            for (a, b) in a.iter().zip(b.iter()) {
                let a = VALUES
                    .iter()
                    .find(|(v, _)| a == v)
                    .map(|(_, v)| *v)
                    .unwrap();
                let b = VALUES
                    .iter()
                    .find(|(v, _)| b == v)
                    .map(|(_, v)| *v)
                    .unwrap();

                match a.cmp(&b) {
                    Ordering::Equal => (),
                    v => return v,
                }
            }

            Ordering::Equal
        }
        v => v,
    }
}

fn cmp_2(a: &[u8; 5], b: &[u8; 5]) -> Ordering {
    const VALUES: &[(u8, u8)] = &[
        (b'J', 1),
        (b'2', 2),
        (b'3', 3),
        (b'4', 4),
        (b'5', 5),
        (b'6', 6),
        (b'7', 7),
        (b'8', 8),
        (b'9', 9),
        (b'T', 10),
        (b'Q', 11),
        (b'K', 12),
        (b'A', 13),
    ];

    match order_2(a).cmp(&order_2(b)) {
        Ordering::Equal => {
            for (a, b) in a.iter().zip(b.iter()) {
                let a = VALUES
                    .iter()
                    .find(|(v, _)| a == v)
                    .map(|(_, v)| *v)
                    .unwrap();
                let b = VALUES
                    .iter()
                    .find(|(v, _)| b == v)
                    .map(|(_, v)| *v)
                    .unwrap();

                match a.cmp(&b) {
                    Ordering::Equal => (),
                    v => return v,
                }
            }

            Ordering::Equal
        }
        v => v,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    High,     // 12345
    Pair,     // 11234
    TwoPairs, // 11223
    Three,    // 11123
    House,    // 11122
    Four,     // 11112
    Five,     // 11111
}

fn order_1(cards: &[u8; 5]) -> Order {
    let mut cards = cards
        .iter()
        .copied()
        .counts()
        .into_values()
        .collect::<Vec<_>>()
        .tap_mut(|v| v.sort_unstable());

    match cards.pop().unwrap() {
        5 => Order::Five,
        4 => Order::Four,
        3 if cards.pop().unwrap() == 2 => Order::House,
        3 => Order::Three,
        2 if cards.pop().unwrap() == 2 => Order::TwoPairs,
        2 => Order::Pair,
        _ => Order::High,
    }
}

fn order_2(cards: &[u8; 5]) -> Order {
    let jokers = cards.iter().filter(|v| **v == b'J').count();

    let mut cards = cards
        .iter()
        .filter(|v| **v != b'J')
        .copied()
        .counts()
        .into_values()
        .collect::<Vec<_>>()
        .tap_mut(|v| v.sort_unstable());

    if let Some(n) = cards.pop() {
        match n + jokers {
            5 => Order::Five,
            4 => Order::Four,
            3 if cards.pop().unwrap() == 2 => Order::House,
            3 => Order::Three,
            2 if cards.pop().unwrap() == 2 => Order::TwoPairs,
            2 => Order::Pair,
            _ => Order::High,
        }
    } else {
        Order::Five
    }
}
