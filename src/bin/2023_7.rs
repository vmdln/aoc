use std::{cmp::Ordering, collections::HashMap, sync::OnceLock};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../assets/2023/07.txt");
    let parsed = parse(input);

    let part_1 = part_1(&parsed);
    println!("1 - `{part_1}`");

    let part_2 = part_2(&parsed);
    println!("2 - `{part_2}`");
}

pub struct Hand<'a> {
    pub cards: &'a [u8; 5],
    pub bid: u64,
    pub order: Order,
}

fn parse(input: &str) -> Vec<([u8; 5], u64)> {
    let mut ret = Vec::new();

    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();

        let cards = cards.as_bytes().to_vec().try_into().unwrap();
        let bid = bid.parse().unwrap();

        ret.push((cards, bid))
    }

    ret
}

fn part_1(parsed: &[([u8; 5], u64)]) -> u64 {
    parsed
        .iter()
        .map(|(cards, bid)| {
            let order = order_1(cards);
            Hand {
                cards,
                bid: *bid,
                order,
            }
        })
        .sorted_unstable_by(cmp_1)
        .zip(1..)
        .map(|(Hand { bid, .. }, rank)| bid * rank)
        .sum()
}

fn part_2(parsed: &[([u8; 5], u64)]) -> u64 {
    parsed
        .iter()
        .map(|(cards, bid)| {
            let order = order_2(cards);
            Hand {
                cards,
                bid: *bid,
                order,
            }
        })
        .sorted_unstable_by(cmp_2)
        .zip(1..)
        .map(|(Hand { bid, .. }, rank)| bid * rank)
        .sum()
}

fn cmp_1(a: &Hand, b: &Hand) -> Ordering {
    static CELL: OnceLock<HashMap<u8, u8>> = OnceLock::new();
    let values = CELL.get_or_init(|| {
        HashMap::from([
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
        ])
    });

    match a.order.cmp(&b.order) {
        Ordering::Equal => {
            for (a, b) in a.cards.iter().zip(b.cards.iter()) {
                let a = values.get(a).unwrap();
                let b = values.get(b).unwrap();

                match a.cmp(b) {
                    Ordering::Equal => (),
                    v => return v,
                }
            }

            Ordering::Equal
        }
        v => v,
    }
}

fn cmp_2(a: &Hand, b: &Hand) -> Ordering {
    static CELL: OnceLock<HashMap<u8, u8>> = OnceLock::new();
    let values = CELL.get_or_init(|| {
        HashMap::from([
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
        ])
    });

    match a.order.cmp(&b.order) {
        Ordering::Equal => {
            for (a, b) in a.cards.iter().zip(b.cards.iter()) {
                let a = values.get(a).unwrap();
                let b = values.get(b).unwrap();

                match a.cmp(b) {
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
        .sorted_unstable()
        .collect_vec();

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
        .sorted_unstable()
        .collect_vec();

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
