use rustc_hash::FxHashSet;
use crate::util::point::Point;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Guard {
    direction: u8,
    position: Point,
}

pub fn part_1(input: String) -> i32 {
    let (grid, mut guard) = parse_grid(input);
    let mut positions = FxHashSet::with_capacity_and_hasher(10_000, Default::default());
    positions.insert(guard.position);

    loop {
        let next = guard.position + Point::from(guard.direction);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    b'.' => {
                        guard.position = next;
                        positions.insert(next);
                    }
                    b'#' => guard.direction = turn(guard.direction),
                    _ => unreachable!(),
                }

                continue;
            }
        }

        break;
    }

    positions.len() as i32
}

pub fn part_2(input: String) -> i32 {
    let (mut grid, mut guard) = parse_grid(input);
    let mut positions = FxHashSet::with_capacity_and_hasher(10_000, Default::default());
    let mut obstacles = 0;

    loop {
        let next = guard.position + Point::from(guard.direction);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    b'.' => {
                        if !positions.contains(&next) {
                            grid[next.y as usize][next.x as usize] = b'#';

                            if is_loop(&grid, guard) {
                                obstacles += 1;
                            }

                            grid[next.y as usize][next.x as usize] = b'.';
                        }

                        guard.position = next;
                        positions.insert(next);
                    }
                    b'#' => guard.direction = turn(guard.direction),
                    _ => unreachable!(),
                }

                continue;
            }
        }

        break;
    }

    obstacles
}

fn parse_grid(input: String) -> (Vec<Vec<u8>>, Guard) {
    let mut guard = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, b)| {
                    if b == b'^' {
                        guard = Some(Guard {
                            direction: b,
                            position: Point::new(x as i32, y as i32),
                        });

                        b'.'
                    } else {
                        b
                    }
                })
                .collect()
        })
        .collect();

    (grid, guard.unwrap())
}

fn turn(direction: u8) -> u8 {
    match direction {
        b'^' => b'>',
        b'>' => b'v',
        b'v' => b'<',
        b'<' => b'^',
        _ => unreachable!(),
    }
}

fn is_loop(grid: &[Vec<u8>], mut guard: Guard) -> bool {
    let mut turns = FxHashSet::with_capacity_and_hasher(500, Default::default());

    loop {
        let next = guard.position + Point::from(guard.direction);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    b'.' => guard.position = next,
                    b'#' => {
                        guard.direction = turn(guard.direction);

                        if turns.contains(&guard) {
                            return true;
                        }

                        turns.insert(guard);
                    }
                    _ => unreachable!(),
                }

                continue;
            }
        }

        break;
    }

    false
}