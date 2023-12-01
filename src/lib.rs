#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod y2023;

use std::fmt::Display;

pub fn print_results<A, B>(prefix: &str, a: A, b: B)
where
    A: Display,
    B: Display,
{
    println!("{prefix}:");
    println!("- a: {a}");
    println!("- b: {b}");
}
