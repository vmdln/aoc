#![warn(clippy::pedantic)]

use aoc::Grid;
use itertools::Itertools;
use nalgebra::Vector2;
use tap::Pipe as _;

fn main() {
    let input = include_str!("../../assets/2023/10.txt");
    let parsed = parse(input);

    let (part_1, part_2) = solve(&parsed);

    println!("1 - `{part_1}`");
    println!("2 - `{part_2}`");
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pipe {
    #[default]
    None,
    Start,
    Horizontal,
    Vertical,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl TryFrom<u8> for Pipe {
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
    start: Vector2<usize>,
    map: Grid<Pipe>,
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
                    let direction = Pipe::try_from(v).unwrap();
                    if matches!(direction, Pipe::Start) {
                        assert!(start.is_none());
                        start = Some(Vector2::new(x, y));
                    }
                    direction
                })
                .collect_vec()
        })
        .collect_vec()
        .pipe(Grid::try_from)
        .unwrap();

    Parsed {
        start: start.unwrap(),
        map,
    }
}

fn solve(parsed: &Parsed) -> (u64, u64) {
    let mut iter = [
        (Direction::Left, parsed.map.get_left(parsed.start)),
        (Direction::Right, parsed.map.get_right(parsed.start)),
        (Direction::Down, parsed.map.get_down(parsed.start)),
        (Direction::Up, parsed.map.get_up(parsed.start)),
    ]
    .into_iter()
    .filter_map(|(dir, next)| next.and_then(|v| if is_valid(&dir, v) { Some(dir) } else { None }));

    let mut current = parsed.start;
    let mut dir = iter.next().unwrap();

    let mut visited = Grid::new_default(parsed.map.size()).unwrap();
    *visited.get_mut(current).unwrap() = match (dir, iter.next().unwrap()) {
        (Direction::Down, Direction::Up) => Pipe::Vertical,
        (Direction::Right, Direction::Up) => Pipe::UpRight,
        (Direction::Right, Direction::Down) => Pipe::DownRight,
        (Direction::Left, Direction::Up) => Pipe::UpLeft,
        (Direction::Left, Direction::Down) => Pipe::DownLeft,
        (Direction::Left, Direction::Right) => Pipe::Horizontal,
        _ => panic!(), // the other ones aren't possible
    };

    let mut part_1 = 0;
    loop {
        part_1 += 1;

        dir = update(&mut current, &parsed.map, &dir);
        let pipe = parsed.map.get(current).unwrap();
        if pipe == &Pipe::Start {
            break;
        }
        *visited.get_mut(current).unwrap() = *pipe;
    }

    let mut part_2 = 0;
    for row in visited.rows() {
        let mut acc_top = false;
        let mut acc_bot = false;
        for cell in row {
            match cell {
                Pipe::None => {
                    if acc_top || acc_bot {
                        part_2 += 1;
                    }
                }
                Pipe::Start => panic!(),
                Pipe::Horizontal => (),
                Pipe::Vertical => {
                    acc_top = !acc_top;
                    acc_bot = !acc_bot;
                }
                Pipe::UpRight | Pipe::UpLeft => {
                    acc_top = !acc_top;
                }
                Pipe::DownRight | Pipe::DownLeft => {
                    acc_bot = !acc_bot;
                }
            }
        }
    }

    (part_1 / 2, part_2)
}

pub fn update(pos: &mut Vector2<usize>, map: &Grid<Pipe>, direction: &Direction) -> Direction {
    match direction {
        Direction::Up => {
            pos.y -= 1;
        }
        Direction::Down => {
            pos.y += 1;
        }
        Direction::Right => {
            pos.x += 1;
        }
        Direction::Left => {
            pos.x -= 1;
        }
    }

    let pipe = *map.get(*pos).unwrap();

    match pipe {
        Pipe::None => panic!("{pos}"),
        Pipe::Start => *direction,
        Pipe::Horizontal => match direction {
            Direction::Up => panic!(),
            Direction::Down => panic!(),
            Direction::Right => *direction,
            Direction::Left => *direction,
        },
        Pipe::Vertical => match direction {
            Direction::Up => *direction,
            Direction::Down => *direction,
            Direction::Right => panic!(),
            Direction::Left => panic!(),
        },
        Pipe::UpRight => match direction {
            Direction::Up => panic!(),
            Direction::Down => Direction::Right,
            Direction::Right => panic!(),
            Direction::Left => Direction::Up,
        },
        Pipe::UpLeft => match direction {
            Direction::Up => panic!(),
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Left => panic!(),
        },
        Pipe::DownRight => match direction {
            Direction::Up => Direction::Right,
            Direction::Down => panic!(),
            Direction::Right => panic!(),
            Direction::Left => Direction::Down,
        },
        Pipe::DownLeft => match direction {
            Direction::Up => Direction::Left,
            Direction::Down => panic!(),
            Direction::Right => Direction::Down,
            Direction::Left => panic!(),
        },
    }
}

pub fn is_valid(direction: &Direction, pipe: &Pipe) -> bool {
    match direction {
        Direction::Up => match pipe {
            Pipe::None => false,
            Pipe::Start => true,
            Pipe::Horizontal => false,
            Pipe::Vertical => true,
            Pipe::UpRight => false,
            Pipe::UpLeft => false,
            Pipe::DownRight => true,
            Pipe::DownLeft => true,
        },
        Direction::Down => match pipe {
            Pipe::None => false,
            Pipe::Start => true,
            Pipe::Horizontal => false,
            Pipe::Vertical => true,
            Pipe::UpRight => true,
            Pipe::UpLeft => true,
            Pipe::DownRight => false,
            Pipe::DownLeft => false,
        },
        Direction::Right => match pipe {
            Pipe::None => false,
            Pipe::Start => true,
            Pipe::Horizontal => true,
            Pipe::Vertical => false,
            Pipe::UpRight => false,
            Pipe::UpLeft => true,
            Pipe::DownRight => false,
            Pipe::DownLeft => true,
        },
        Direction::Left => match pipe {
            Pipe::None => false,
            Pipe::Start => true,
            Pipe::Horizontal => true,
            Pipe::Vertical => false,
            Pipe::UpRight => true,
            Pipe::UpLeft => false,
            Pipe::DownRight => true,
            Pipe::DownLeft => false,
        },
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
