use std::{cell::Cell, collections::{HashMap, HashSet, VecDeque}, fmt::{Debug, Display}, time::Duration};
use itertools::Itertools;


#[derive(PartialEq, Debug)]
enum Move {
    Up, Down, Right, Left
}

impl Move {
    fn next(&self) -> (i32, i32) {
        match self {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Right => (0, 1),
            Move::Left => (0, -1),
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value {
            "^" => Move::Up,
            "v" => Move::Down,
            ">" => Move::Right,
            "<" => Move::Left,
            _ => panic!("Invalid from: {value:?}"),
        }
    }
}

pub fn part_1(input: String) -> i32 {
    #[derive(PartialEq, Debug)]
    enum Cell {
        Robot, Wall, Box, Empty
    }

    let (matrix, moves) = input
        .split("\n\n")
        .map(|s| s.to_string())
        .next_tuple::<(String, String)>()
        .unwrap();

    let mut robot = (-1, -1);
    let mut matrix: Vec<Vec<Cell>> = matrix
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let line = line
                .split("")
                .filter(|s| !s.is_empty())
                .enumerate()
                .map(|(x, s)| {
                    match s.chars().next().unwrap() {
                        '@' => { robot = (y as i32, x as i32); Cell::Robot },
                        'O' => Cell::Box,
                        '#' => Cell::Wall,
                        '.' => Cell::Empty,
                        _ => unreachable!()
                    }
                })
                .collect::<Vec<Cell>>();

            matrix.push(line);
            matrix
        });

    let mut moves: VecDeque<Move> = moves
        .split("\n")
        .join("")
        .split("")
        .filter(|s| !s.is_empty())
        .map(|each| Move::from(each))
        .collect();

    let boxes = |m: Move, start: (i32, i32), matrix: &Vec<Vec<Cell>>| -> Option<(i32, i32)> {
        let mut start = start;
        let mut end = None;

        loop {
            let next = m.next();
            let next = (start.0 + next.0, start.1 + next.1);

            match matrix[next.0 as usize][next.1 as usize] {
                Cell::Box => start = next,
                Cell::Wall | Cell::Robot => break,
                Cell::Empty => {
                    end = Some(next);
                    break;
                },
            }
        }

        end
    };

    while let Some(m) = moves.pop_front() {
        let next = m.next();
        let next = (robot.0 + next.0, robot.1 + next.1);

        match matrix[next.0 as usize][next.1 as usize] {
            Cell::Robot | Cell::Wall => {},
            Cell::Empty => {
                matrix[robot.0 as usize][robot.1 as usize] = Cell::Empty;
                matrix[next.0 as usize][next.1 as usize] = Cell::Robot;
                robot = next;
            },
            Cell::Box => {
                if let Some(end) = boxes(m, next, &matrix) {
                    matrix[end.0 as usize][end.1 as usize] = Cell::Box;
                    matrix[robot.0 as usize][robot.1 as usize] = Cell::Empty;
                    matrix[next.0 as usize][next.1 as usize] = Cell::Robot;
                    robot = next;
                }
            },
        }
    }

    // matrix.iter().for_each(|l| println!("{:?}", l.iter().join("")));

    let mut k = 0;
    for y in 1..matrix.len() {
        for x in 1..matrix[y].len() {
            if matrix[y][x] == Cell::Box {
                k += (100 * y) + x
            }
        }
    }

    k as i32
}

pub fn part_2(input: String) -> i32 {
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum Cell {
        Robot,
        Wall,
        Empty,
        Box(u32),
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Cell::Robot => "@",
                Cell::Wall => "■",
                Cell::Empty => ".",
                Cell::Box(_) => "#",
            })
        }
    }

    let (matrix, moves) = input
        .split("\n\n")
        .map(|s| s.to_string())
        .next_tuple::<(String, String)>()
        .unwrap();

    let mut boxes = 0;
    let mut robot = (-1, -1);
    let mut matrix: Vec<Vec<Cell>> = matrix
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let line = line
                .split("")
                .filter(|s| !s.is_empty())
                .enumerate()
                .flat_map(|(x, s)| {
                    match s.chars().next().unwrap() {
                        '@' => {
                            robot = (y as i32, x as i32 * 2);
                            vec![Cell::Robot, Cell::Empty]
                        },
                        'O' => {
                            boxes += 1;
                            vec![Cell::Box(boxes), Cell::Box(boxes)]
                        },
                        '#' => vec![Cell::Wall, Cell::Wall],
                        '.' => vec![Cell::Empty, Cell::Empty],
                        _ => unreachable!()
                    }
                })
                .collect();

            matrix.push(line);
            matrix
        });

    let mut moves: VecDeque<Move> = moves
        .split("\n")
        .join("")
        .split("")
        .filter(|s| !s.is_empty())
        .map(|each| Move::from(each))
        .collect();

    fn can_be_moved_y(m: &Move, box_id: u32, a: (i32, i32), matrix: &mut Vec<Vec<Cell>>) -> bool {
        let (b, b_position) = if matrix[a.0 as usize][a.1 as usize - 1] == Cell::Box(box_id) {
            ((a.0, a.1 - 1), Move::Left)
        } else if matrix[a.0 as usize][a.1 as usize + 1] == Cell::Box(box_id) {
            ((a.0, a.1 + 1), Move::Right)
        } else {
            unreachable!()
        };

        let next: (i32, i32) = m.next();
        let a_next = (a.0 + next.0, a.1 + next.1);
        let b_next = (b.0 + next.0, b.1 + next.1);

        let mut check = Vec::new();
        let all = Vec::from([
            (a, a_next),
            (b, b_next),
        ]);

        match m {
            Move::Up | Move::Down => {
                check.extend(all.clone());
            },
            Move::Right | Move::Left => {
                if b_position == *m {
                    check.push((b, b_next));
                } else {
                    check.push((a, a_next));
                }
            }
        }

        let can = check
            .iter()
            .all(|(_, next)| match matrix[next.0 as usize][next.1 as usize] {
                Cell::Empty => true,
                Cell::Robot | Cell::Wall => false,
                Cell::Box(next_id) => can_be_moved_y(m, next_id, *next, matrix),
            });

        if can {
            all.iter().for_each(|(from, _)| matrix[from.0 as usize][from.1 as usize] = Cell::Empty);
            all.iter().for_each(|(_, to)| matrix[to.0 as usize][to.1 as usize] = Cell::Box(box_id));
        }

        can
    };

    let mut i = 0;
    while let Some(m) = moves.pop_front() {
        let next = m.next();
        let next = (robot.0 + next.0, robot.1 + next.1);

        let mut is_moved = false;
        match matrix[next.0 as usize][next.1 as usize] {
            Cell::Robot | Cell::Wall => {},
            Cell::Empty => {
                matrix[robot.0 as usize][robot.1 as usize] = Cell::Empty;
                matrix[next.0 as usize][next.1 as usize] = Cell::Robot;
                robot = next;
            },
            Cell::Box(id) => {
                if can_be_moved_y(&m, id, next, &mut matrix) {
                    // println!("- [moved] box: {robot:?}; next: {next:?}; box: {id}");
                    matrix[next.0 as usize][next.1 as usize] = Cell::Robot;
                    matrix[robot.0 as usize][robot.1 as usize] = Cell::Empty;
                    robot = next;
                    is_moved = true;
                }
            },
        }

        i += 1;
        if i > 1000 {
            std::process::Command::new("clear").status().unwrap();
            println!("[{i}] Move: {m:?}");
            matrix.iter().for_each(|l| {
                let mut m = HashSet::new();
                println!("{}", l.iter().map(|cell| {
                    match cell {
                        Cell::Box(id) => {
                            if m.contains(id) {
                                format!("]")
                            } else {
                                m.insert(*id);
                                format!("[")
                            }
                        },
                        _ => format!("{cell}")
                    }
                }).join(""))
            });
            std::thread::sleep(Duration::from_millis(if is_moved { 1000 } else { 200 }));
        }
    }

    let mut k = 0;
    for y in 1..matrix.len() {
        let yc = 100 * y;
        k += matrix[y]
            .iter()
            .enumerate()
            // .skip(2)
            .filter_map(|(x, cell)| match cell {
                Cell::Box(id) => Some((*id, x)),
                _ => None,
            })
            .fold(HashMap::new(), |mut acc: HashMap<u32, usize>, (n, x)| {
                acc.entry(n).or_insert(x);
                acc
            })
            .iter()
            .map(|(_, x)| yc + x)
            .sum::<usize>()
    }

    k as i32
}