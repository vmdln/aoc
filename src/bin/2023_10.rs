#![warn(clippy::pedantic)]

use aoc::Grid;
use image::Rgb;
use itertools::Itertools;
use nalgebra::Vector2;
use tap::Pipe as _;

fn main() {
    let input = include_str!("../../assets/2023/10.txt");
    let parsed = parse(input);

    let part_1 = part_1(&parsed);
    println!("1 - `{part_1}`");

    let part_2 = part_2(&parsed);
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

fn part_1(parsed: &Parsed) -> u64 {
    let mut iter = [
        (Direction::Left, parsed.map.get_left(parsed.start)),
        (Direction::Right, parsed.map.get_right(parsed.start)),
        (Direction::Down, parsed.map.get_down(parsed.start)),
        (Direction::Up, parsed.map.get_up(parsed.start)),
    ]
    .into_iter()
    .filter_map(|(dir, next)| next.and_then(|v| if is_valid(&dir, v) { Some(dir) } else { None }));

    let mut a_cur = parsed.start;
    let mut b_cur = parsed.start;

    let mut a_dir = iter.next().unwrap();
    let mut b_dir = iter.next().unwrap();

    let mut part_1 = 0;
    loop {
        part_1 += 1;

        let a_prev = a_cur;

        a_dir = update(&mut a_cur, &parsed.map, &a_dir);
        b_dir = update(&mut b_cur, &parsed.map, &b_dir);

        if a_cur == b_cur || a_prev == b_cur {
            break;
        }
    }

    part_1
}

fn part_2(parsed: &Parsed) -> u64 {
    let zoomed = zoom(&parsed.map);
    let start = parsed.start * 3 + Vector2::new(1, 1);

    let mut iter = [
        (Direction::Left, zoomed.get_left(start)),
        (Direction::Right, zoomed.get_right(start)),
        (Direction::Down, zoomed.get_down(start)),
        (Direction::Up, zoomed.get_up(start)),
    ]
    .into_iter()
    .filter_map(|(dir, next)| next.and_then(|v| if is_valid(&dir, v) { Some(dir) } else { None }));

    let mut a_cur = start;
    let mut b_cur = start;

    let mut a_dir = iter.next().unwrap();
    let mut b_dir = iter.next().unwrap();

    let mut visited = Grid::<State>::new_default(zoomed.size()).unwrap();
    *visited.get_mut(start).unwrap() = State::Blocked;

    loop {
        let a_prev = a_cur;

        a_dir = update(&mut a_cur, &zoomed, &a_dir);
        *visited.get_mut(a_cur).unwrap() = State::Blocked;

        b_dir = update(&mut b_cur, &zoomed, &b_dir);
        *visited.get_mut(b_cur).unwrap() = State::Blocked;

        if a_cur == b_cur || a_prev == b_cur {
            break;
        }
    }

    *visited.get_mut(Vector2::new(0, 0)).unwrap() = State::Filled;

    loop {
        let mut changed = false;
        for (y, x) in (0..visited.size().y).cartesian_product(0..visited.size().x) {
            let pos = Vector2::new(x, y);
            if visited.get_mut(pos).unwrap() == &State::Empty {
                if visited
                    .get_up(pos)
                    .map_or(false, |cell| cell == &State::Filled)
                {
                    *visited.get_mut(pos).unwrap() = State::Filled;
                    changed = true;
                    break;
                }
                if visited
                    .get_down(pos)
                    .map_or(false, |cell| cell == &State::Filled)
                {
                    *visited.get_mut(pos).unwrap() = State::Filled;
                    changed = true;
                    break;
                }
                if visited
                    .get_right(pos)
                    .map_or(false, |cell| cell == &State::Filled)
                {
                    *visited.get_mut(pos).unwrap() = State::Filled;
                    changed = true;
                    break;
                }
                if visited
                    .get_left(pos)
                    .map_or(false, |cell| cell == &State::Filled)
                {
                    *visited.get_mut(pos).unwrap() = State::Filled;
                    changed = true;
                    break;
                }
            }
        }

        if !changed {
            break;
        }
    }

    visited.save_image("out2.png").unwrap();

    let mut part_2 = 0;
    for y in (0..zoomed.size().y).skip(1).step_by(3) {
        for x in (0..zoomed.size().x).skip(1).step_by(3) {
            if visited.get(Vector2::new(x, y)).unwrap() == &State::Empty {
                part_2 += 1;
            }
        }
    }
    part_2
}

pub fn zoom(grid: &Grid<Pipe>) -> Grid<Pipe> {
    let size = grid.size() * 3;
    let mut new = Grid::new_default(size).unwrap();

    for (y, x) in grid
        .size()
        .pipe(|size| (0..size.y).cartesian_product(0..size.x))
    {
        let pos = Vector2::new(x, y);
        match grid.get(pos).unwrap() {
            Pipe::None => (),
            Pipe::Start => {
                if grid
                    .get_left(pos)
                    .and_then(|pipe| {
                        if is_valid(&Direction::Left, pipe) {
                            Some(pipe)
                        } else {
                            None
                        }
                    })
                    .is_some()
                {
                    *new.get_mut_left(Vector2::new(x * 3 + 1, y * 3 + 1))
                        .unwrap() = Pipe::Horizontal;
                }
                if grid
                    .get_right(pos)
                    .and_then(|pipe| {
                        if is_valid(&Direction::Right, pipe) {
                            Some(pipe)
                        } else {
                            None
                        }
                    })
                    .is_some()
                {
                    *new.get_mut_right(Vector2::new(x * 3 + 1, y * 3 + 1))
                        .unwrap() = Pipe::Horizontal;
                }
                if grid
                    .get_down(pos)
                    .and_then(|pipe| {
                        if is_valid(&Direction::Down, pipe) {
                            Some(pipe)
                        } else {
                            None
                        }
                    })
                    .is_some()
                {
                    *new.get_mut_down(Vector2::new(x * 3 + 1, y * 3 + 1))
                        .unwrap() = Pipe::Vertical;
                }
                if grid
                    .get_up(pos)
                    .and_then(|pipe| {
                        if is_valid(&Direction::Up, pipe) {
                            Some(pipe)
                        } else {
                            None
                        }
                    })
                    .is_some()
                {
                    *new.get_mut_up(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::Vertical;
                }
                *new.get_mut(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::Start;
            }
            Pipe::Horizontal => {
                *new.get_mut_left(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Horizontal;
                *new.get_mut(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::Horizontal;
                *new.get_mut_right(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Horizontal;
            }
            Pipe::Vertical => {
                *new.get_mut_up(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::Vertical;
                *new.get_mut(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::Vertical;
                *new.get_mut_down(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Vertical;
            }
            Pipe::UpRight => {
                *new.get_mut_up(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::Vertical;
                *new.get_mut(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::UpRight;
                *new.get_mut_right(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Horizontal;
            }
            Pipe::UpLeft => {
                *new.get_mut_up(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::Vertical;
                *new.get_mut(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::UpLeft;
                *new.get_mut_left(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Horizontal;
            }
            Pipe::DownRight => {
                *new.get_mut_down(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Vertical;
                *new.get_mut(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::DownRight;
                *new.get_mut_right(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Horizontal;
            }
            Pipe::DownLeft => {
                *new.get_mut_down(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Vertical;
                *new.get_mut(Vector2::new(x * 3 + 1, y * 3 + 1)).unwrap() = Pipe::DownLeft;
                *new.get_mut_left(Vector2::new(x * 3 + 1, y * 3 + 1))
                    .unwrap() = Pipe::Horizontal;
            }
        }
    }

    new
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
        Pipe::Start => panic!(),
        pipe @ Pipe::Horizontal => match direction {
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

#[derive(PartialEq, Eq, Clone, Copy, Default)]
enum State {
    #[default]
    Empty,
    Blocked,
    Filled,
}

impl From<&State> for Rgb<u8> {
    fn from(value: &State) -> Self {
        match value {
            State::Empty => Rgb([0x00, 0x00, 0x00]),
            State::Blocked => Rgb([0xff, 0xff, 0xff]),
            State::Filled => Rgb([0x40, 0x40, 0x40]),
        }
    }
}
