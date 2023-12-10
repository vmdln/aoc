#![warn(clippy::pedantic)]

use itertools::Itertools;
use tap::prelude::*;

fn main() {
    let input = include_str!("../../assets/2023/10.txt");
    let parsed = parse(input);

    let part_1 = part_1(&parsed);
    println!("1 - `{part_1}`");

    let part_2 = part_2(&parsed);
    println!("2 - `{part_2}`");
}

#[derive(Clone)]
pub enum Direction {
    None,
    Start,
    Horizontal,
    Vertical,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let ret = match value {
            b'.' => Self::None,
            b'S' => Self::Start,
            b'-' => Self::Horizontal,
            b'|' => Self::Vertical,
            b'L' => Self::UpRight,
            b'J' => Self::UpLeft,
            b'F' => Self::DownRight,
            b'7' => Self::DownLeft,
            _ => return Err(()),
        };

        Ok(ret)
    }
}

pub struct Parsed {
    start: Pos,
    map: Vec<Vec<Direction>>,
}

fn parse(input: &str) -> Parsed {
    let mut start = None;

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, v)| {
                    let direction = Direction::try_from(v).unwrap();
                    if matches!(direction, Direction::Start) {
                        start = Some(Pos { x, y });
                    }
                    direction
                })
                .collect_vec()
        })
        .collect_vec();

    Parsed {
        start: start.unwrap(),
        map,
    }
}

pub fn part_1(parsed: &Parsed) -> u64 {
    let start = parsed.start.clone();

    let mut iter = [
        (get_left(&start, &parsed.map), MovementDirection::Left),
        (get_right(&start, &parsed.map), MovementDirection::Right),
        (get_down(&start, &parsed.map), MovementDirection::Down),
        (get_up(&start, &parsed.map), MovementDirection::Up),
    ]
    .into_iter()
    .filter_map(|(v, direction)| v.map(|_| direction));
    let mut a_dir = iter.next().unwrap();
    let mut b_dir = iter.next().unwrap();

    let mut current_a = start.clone();
    let mut current_b = start.clone();
    let mut previous_a = current_a.clone();
    let mut previous_b = current_b.clone();

    let mut steps = 0_u64;

    loop {
        let previous_a = current_a.clone();
        match a_dir {
            MovementDirection::Up => current_a.y -= 1,
            MovementDirection::Down => current_a.y += 1,
            MovementDirection::Right => current_a.x += 1,
            MovementDirection::Left => current_a.x -= 1,
        }

        match get(&current_a, &parsed.map) {
            Direction::None => panic!(),
            Direction::Start => panic!("{}", steps),
            Direction::Horizontal => match a_dir {
                MovementDirection::Up => panic!(),
                MovementDirection::Down => panic!(),
                MovementDirection::Right => (),
                MovementDirection::Left => (),
            },
            Direction::Vertical => match a_dir {
                MovementDirection::Up => (),
                MovementDirection::Down => (),
                MovementDirection::Right => panic!(),
                MovementDirection::Left => panic!(),
            },
            Direction::UpRight => match a_dir {
                MovementDirection::Up => panic!(),
                MovementDirection::Down => a_dir = MovementDirection::Right,
                MovementDirection::Right => panic!(),
                MovementDirection::Left => a_dir = MovementDirection::Up,
            },
            Direction::UpLeft => match a_dir {
                MovementDirection::Up => panic!(),
                MovementDirection::Down => a_dir = MovementDirection::Left,
                MovementDirection::Left => panic!(),
                MovementDirection::Right => a_dir = MovementDirection::Up,
            },
            Direction::DownRight => match a_dir {
                MovementDirection::Up => a_dir = MovementDirection::Right,
                MovementDirection::Down => panic!(),
                MovementDirection::Left => a_dir = MovementDirection::Down,
                MovementDirection::Right => panic!(),
            },
            Direction::DownLeft => match a_dir {
                MovementDirection::Up => a_dir = MovementDirection::Left,
                MovementDirection::Down => panic!(),
                MovementDirection::Right => a_dir = MovementDirection::Down,
                MovementDirection::Left => panic!("{:?}", current_a),
            },
        }

        let previous_b = current_b.clone();
        match b_dir {
            MovementDirection::Up => current_b.y -= 1,
            MovementDirection::Down => current_b.y += 1,
            MovementDirection::Right => current_b.x += 1,
            MovementDirection::Left => current_b.x -= 1,
        }

        match get(&current_b, &parsed.map) {
            Direction::None => panic!(),
            Direction::Start => panic!(),
            Direction::Horizontal => match b_dir {
                MovementDirection::Up => panic!(),
                MovementDirection::Down => panic!(),
                MovementDirection::Right => (),
                MovementDirection::Left => (),
            },
            Direction::Vertical => match b_dir {
                MovementDirection::Up => (),
                MovementDirection::Down => (),
                MovementDirection::Right => panic!(),
                MovementDirection::Left => panic!(),
            },
            Direction::UpRight => match b_dir {
                MovementDirection::Up => panic!(),
                MovementDirection::Down => b_dir = MovementDirection::Right,
                MovementDirection::Right => panic!(),
                MovementDirection::Left => b_dir = MovementDirection::Up,
            },
            Direction::UpLeft => match b_dir {
                MovementDirection::Up => panic!(),
                MovementDirection::Down => b_dir = MovementDirection::Left,
                MovementDirection::Left => panic!(),
                MovementDirection::Right => b_dir = MovementDirection::Up,
            },
            Direction::DownRight => match b_dir {
                MovementDirection::Up => b_dir = MovementDirection::Right,
                MovementDirection::Down => panic!(),
                MovementDirection::Left => b_dir = MovementDirection::Down,
                MovementDirection::Right => panic!(),
            },
            Direction::DownLeft => match b_dir {
                MovementDirection::Up => b_dir = MovementDirection::Left,
                MovementDirection::Down => panic!(),
                MovementDirection::Right => b_dir = MovementDirection::Down,
                MovementDirection::Left => panic!(),
            },
        }

        steps += 1;
        if current_a == current_b || previous_a == current_b || previous_b == current_a {
            return steps;
        }
    }
}

#[derive(Clone, Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

pub fn get(pos: &Pos, map: &[Vec<Direction>]) -> Direction {
    map[pos.y].get(pos.x).cloned().unwrap()
}

pub fn get_left(pos: &Pos, map: &[Vec<Direction>]) -> Option<Direction> {
    pos.x.checked_sub(1).and_then(|x| {
        let v = map[pos.y].get(x).cloned().unwrap();
        match v {
            v @ (Direction::Horizontal | Direction::UpRight | Direction::DownRight) => Some(v),
            _ => None,
        }
    })
}

pub fn get_right(pos: &Pos, map: &[Vec<Direction>]) -> Option<Direction> {
    let x = pos.x + 1;
    map[pos.y].get(x).cloned().and_then(|v| match v {
        v @ (Direction::Horizontal | Direction::UpLeft | Direction::DownLeft) => Some(v),
        _ => None,
    })
}

pub fn get_up(pos: &Pos, map: &[Vec<Direction>]) -> Option<Direction> {
    pos.y
        .checked_sub(1)
        .and_then(|y| match map[y].get(pos.x).cloned().unwrap() {
            v @ (Direction::Vertical | Direction::DownRight | Direction::DownLeft) => Some(v),
            _ => None,
        })
}

pub fn get_down(pos: &Pos, map: &[Vec<Direction>]) -> Option<Direction> {
    let y = pos.y + 1;
    map[y].get(pos.x).cloned().and_then(|v| match v {
        v @ (Direction::Vertical | Direction::UpRight | Direction::UpLeft) => Some(v),
        _ => None,
    })
}

pub fn part_2(parsed: &Parsed) -> u64 {
    let zoomed = zoom(&parsed.map);
    let mut current = parsed.start.pipe_ref(|Pos { x, y }| {
        let x = x * 3 + 1;
        let y = y * 3 + 1;
        Pos { x, y }
    });

    //  for row in &zoomed {
    //      for v in row {
    //          match v {
    //              Direction::None => print!(" "),
    //              Direction::Start => print!("S"),
    //              Direction::Horizontal => print!("-"),
    //              Direction::Vertical => print!("|"),
    //              Direction::UpRight => print!("L"),
    //              Direction::UpLeft => print!("J"),
    //              Direction::DownRight => print!("F"),
    //              Direction::DownLeft => print!("7"),
    //          }
    //      }
    //      println!();
    //  }

    let mut iter = [
        (get_left(&current, &zoomed), MovementDirection::Left),
        (get_right(&current, &zoomed), MovementDirection::Right),
        (get_down(&current, &zoomed), MovementDirection::Down),
        (get_up(&current, &zoomed), MovementDirection::Up),
    ]
    .into_iter()
    .filter_map(|(v, direction)| v.map(|_| direction));
    let mut dir = iter.next().unwrap();

    let mut new_map = zoomed
        .iter()
        .map(|row| row.iter().map(|_| State::Empty).collect_vec())
        .collect_vec();

    loop {
        match dir {
            MovementDirection::Up => current.y -= 1,
            MovementDirection::Down => current.y += 1,
            MovementDirection::Right => current.x += 1,
            MovementDirection::Left => current.x -= 1,
        }

        *new_map[current.y].get_mut(current.x).unwrap() = State::Blocked;
        match get(&current, &zoomed) {
            Direction::None => panic!(),
            Direction::Start => break,
            Direction::Horizontal => match dir {
                MovementDirection::Up => panic!(),
                MovementDirection::Down => panic!(),
                MovementDirection::Right => (),
                MovementDirection::Left => (),
            },
            Direction::Vertical => match dir {
                MovementDirection::Up => (),
                MovementDirection::Down => (),
                MovementDirection::Right => panic!(),
                MovementDirection::Left => panic!("{current:?}"),
            },
            Direction::UpRight => {
                match dir {
                    MovementDirection::Up => panic!(),
                    MovementDirection::Down => dir = MovementDirection::Right,
                    MovementDirection::Right => panic!(),
                    MovementDirection::Left => dir = MovementDirection::Up,
                }
                //*new_map[current.y - 1].get_mut(current.x + 1).unwrap() = State::Filled;
            }
            Direction::UpLeft => {
                match dir {
                    MovementDirection::Up => panic!(),
                    MovementDirection::Down => dir = MovementDirection::Left,
                    MovementDirection::Left => panic!(),
                    MovementDirection::Right => dir = MovementDirection::Up,
                };
                //*new_map[current.y - 1].get_mut(current.x - 1).unwrap() = State::Filled;
            }
            Direction::DownRight => {
                match dir {
                    MovementDirection::Up => dir = MovementDirection::Right,
                    MovementDirection::Down => panic!(),
                    MovementDirection::Left => dir = MovementDirection::Down,
                    MovementDirection::Right => panic!(),
                }
                //*new_map[current.y + 1].get_mut(current.x + 1).unwrap() = State::Filled;
            }
            Direction::DownLeft => {
                match dir {
                    MovementDirection::Up => dir = MovementDirection::Left,
                    MovementDirection::Down => panic!(),
                    MovementDirection::Right => dir = MovementDirection::Down,
                    MovementDirection::Left => panic!(),
                }
                //*new_map[current.y + 1].get_mut(current.x - 1).unwrap() = State::Filled;
            }
        }
    }

    //  println!("{}", new_map.len());

    //  for row in &new_map {
    //      for v in row {
    //          match v {
    //              State::Empty => eprint!("E"),
    //              State::Blocked => eprint!("#"),
    //              State::Filled => eprint!("f"),
    //          }
    //      }
    //      eprintln!();
    //  }

    fill(&mut new_map)
}

#[derive(PartialEq, Eq)]
enum State {
    Empty,
    Blocked,
    Filled,
}

pub fn fill(map: &mut Vec<Vec<State>>) -> u64 {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y].get(x).unwrap() == &State::Empty {
                *map[y].get_mut(x).unwrap() = State::Filled;
            }
        }
    }
    *map[0].get_mut(0).unwrap() = State::Empty;

    let mut i = 0_u64;
    loop {
        i += 1;
        let mut changed = false;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y].get(x).unwrap() == &State::Empty
                    && y > 0
                    && map[y - 1].get(x).unwrap() == &State::Filled
                {
                    *map[y - 1].get_mut(x).unwrap() = State::Empty;
                    changed = true;
                }

                if map[y].get(x).unwrap() == &State::Empty
                    && y < map.len() - 1
                    && map[y + 1].get(x).unwrap() == &State::Filled
                {
                    *map[y + 1].get_mut(x).unwrap() = State::Empty;
                    changed = true;
                }

                if map[y].get(x).unwrap() == &State::Empty
                    && x > 0
                    && map[y].get(x - 1).unwrap() == &State::Filled
                {
                    *map[y].get_mut(x - 1).unwrap() = State::Empty;
                    changed = true;
                }

                if map[y].get(x).unwrap() == &State::Empty
                    && x < map[y].len() - 1
                    && map[y].get(x + 1).unwrap() == &State::Filled
                {
                    *map[y].get_mut(x + 1).unwrap() = State::Empty;
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut acc = 0_u64;
    for y in 0..map.len() / 3 {
        let y = y * 3 + 1;
        for x in 0..map[0].len() / 3 {
            let x = x * 3 + 1;
            if map[y].get(x).unwrap() == &State::Filled {
                acc += 1;
            }
        }
    }

    acc
}

pub fn zoom(map: &[Vec<Direction>]) -> Vec<Vec<Direction>> {
    let mut zoomed_in = Vec::new();

    for (y, row) in map.into_iter().enumerate() {
        zoomed_in.push(Vec::new());
        zoomed_in.push(Vec::new());
        zoomed_in.push(Vec::new());
        for (x, cell) in row.iter().enumerate() {
            match cell {
                Direction::None => {
                    zoomed_in[y * 3].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 1].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 2].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                }
                Direction::Start => {
                    if get_up(&Pos { x, y }, map).is_some() {
                        zoomed_in[y * 3].extend_from_slice(&[
                            Direction::None,
                            Direction::Vertical,
                            Direction::None,
                        ]);
                    } else {
                        zoomed_in[y * 3].extend_from_slice(&[
                            Direction::None,
                            Direction::None,
                            Direction::None,
                        ]);
                    }

                    if get_left(&Pos { x, y }, map).is_some() {
                        zoomed_in[y * 3 + 1].push(Direction::Horizontal);
                    } else {
                        zoomed_in[y * 3 + 1].push(Direction::None);
                    }

                    zoomed_in[y * 3 + 1].push(Direction::Start);

                    if get_right(&Pos { x, y }, map).is_some() {
                        zoomed_in[y * 3 + 1].push(Direction::Horizontal);
                    } else {
                        zoomed_in[y * 3 + 1].push(Direction::None);
                    }

                    if get_down(&Pos { x, y }, map).is_some() {
                        zoomed_in[y * 3 + 2].extend_from_slice(&[
                            Direction::None,
                            Direction::Vertical,
                            Direction::None,
                        ]);
                    } else {
                        zoomed_in[y * 3 + 2].extend_from_slice(&[
                            Direction::None,
                            Direction::None,
                            Direction::None,
                        ]);
                    }
                }
                Direction::Horizontal => {
                    zoomed_in[y * 3].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 1].extend_from_slice(&[
                        Direction::Horizontal,
                        Direction::Horizontal,
                        Direction::Horizontal,
                    ]);
                    zoomed_in[y * 3 + 2].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                }
                Direction::Vertical => {
                    zoomed_in[y * 3].extend_from_slice(&[
                        Direction::None,
                        Direction::Vertical,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 1].extend_from_slice(&[
                        Direction::None,
                        Direction::Vertical,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 2].extend_from_slice(&[
                        Direction::None,
                        Direction::Vertical,
                        Direction::None,
                    ]);
                }
                Direction::UpRight => {
                    zoomed_in[y * 3].extend_from_slice(&[
                        Direction::None,
                        Direction::Vertical,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 1].extend_from_slice(&[
                        Direction::None,
                        Direction::UpRight,
                        Direction::Horizontal,
                    ]);
                    zoomed_in[y * 3 + 2].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                }
                Direction::UpLeft => {
                    zoomed_in[y * 3].extend_from_slice(&[
                        Direction::None,
                        Direction::Vertical,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 1].extend_from_slice(&[
                        Direction::Horizontal,
                        Direction::UpLeft,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 2].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                }
                Direction::DownRight => {
                    zoomed_in[y * 3].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 1].extend_from_slice(&[
                        Direction::None,
                        Direction::DownRight,
                        Direction::Horizontal,
                    ]);
                    zoomed_in[y * 3 + 2].extend_from_slice(&[
                        Direction::None,
                        Direction::Vertical,
                        Direction::None,
                    ]);
                }
                Direction::DownLeft => {
                    zoomed_in[y * 3].extend_from_slice(&[
                        Direction::None,
                        Direction::None,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 1].extend_from_slice(&[
                        Direction::Horizontal,
                        Direction::DownLeft,
                        Direction::None,
                    ]);
                    zoomed_in[y * 3 + 2].extend_from_slice(&[
                        Direction::None,
                        Direction::Vertical,
                        Direction::None,
                    ]);
                }
            }
        }
    }

    zoomed_in
}
