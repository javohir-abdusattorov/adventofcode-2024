use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, time::Duration};
use itertools::Itertools;

use crate::util::point::Point;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up, Down, Right, Left
}

impl Direction {
    pub fn next(&self, point: Point) -> Point {
        point + match self {
            Direction::Up => Point::up(),
            Direction::Down => Point::down(),
            Direction::Right => Point::right(),
            Direction::Left => Point::left(),
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }

    pub fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }

    pub fn back(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    pub fn print(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Right => '>',
            Direction::Left => '<',
        }
    }
}

#[derive(Debug)]
struct Position {
    point: Point,
    direction: Direction,
    k: i32,
    visited: HashSet<Point>,
}

impl Position {
    fn is_valid(&self, limits: (i32, i32)) -> bool {
        self.point.x >= 0 && self.point.y >= 0 && self.point.x < limits.0 && self.point.y < limits.1
    }

    fn next(&self) -> Position {
        self.go(self.direction, 1)
    }

    fn right(&self) -> Position {
        self.go(self.direction.right(), 1001)
    }

    fn left(&self) -> Position {
        self.go(self.direction.left(), 1001)
    }

    fn go(&self, direction: Direction, k: i32) -> Position {
        let point = direction.next(self.point);
        let mut visited = self.visited.clone();
        visited.insert(point);

        Position {
            point,
            direction,
            visited,
            k: self.k + k,
        }
    }
}


pub fn part_1(input: String) -> i32 {
    let mut matrix = parse(input);
    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;

    let end = Point::new(limit_x - 1, 1);
    let start = Point::new(1, limit_y - 1);

    let sorter = |a: &Position, b: &Position| -> Ordering {
        let k = a.k.cmp(&b.k);
        let manhattan = a.point.manhattan_distance(&end).cmp(&b.point.manhattan_distance(&end));
        let other = (a.point.manhattan_distance(&end) as i32 * a.k).cmp(&(b.point.manhattan_distance(&end) as i32 * b.k));

        k
    };

    let size = 1000000;
    let mut k = i32::MAX;

    let mut queue_2 = Vec::new();
    let mut queue_1 = Vec::from([
        Position {
            point: start,
            direction: Direction::Right,
            k: 0,
            visited: HashSet::default(),
        }
    ]);

    let mut map = HashMap::new();

    let mut i = 0;
    while let Some(position) = queue_1.pop() {
        map.insert(position.point, position.k);
        matrix[position.point.y as usize][position.point.x as usize] = 2;

        if position.point == end {
            k = k.min(position.k);
            continue;
        }

        [position.next(), position.right(), position.left()]
            .into_iter()
            .filter(|next| next.is_valid((limit_x, limit_y)))
            .filter(|next| matrix[next.point.y as usize][next.point.x as usize] != 1)
            .filter(|next| next.k < k)
            .filter(|next| {
                if let Some(last) = map.get(&next.point) { next.k < *last }
                else { true }
            })
            .for_each(|next| queue_2.push(next));

        if queue_1.is_empty() {
            queue_2.sort_by(sorter);
            queue_2.drain(queue_2.len().min(size)..);
            std::mem::swap(&mut queue_1, &mut queue_2);
        }

        i += 1;
        if i % 10000 == 0 {
            println!("");
            println!("");
            println!("[TOP = {k:?}] [SIZE = {}]", queue_2.len());
            matrix.iter().for_each(|line| println!("{}", line.iter().map(|i| match i {
                1 => "#",
                2 => "*",
                _ => " "
            }).join("")));
        }
    }

    k
}

pub fn part_2(input: String) -> i32 {
    let mut matrix = parse(input);
    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;

    let end = Point::new(limit_x - 1, 1);
    let start = Point::new(1, limit_y - 1);

    let sorter = |a: &Position, b: &Position| -> Ordering {
        let k = a.k.cmp(&b.k);
        let manhattan = a.point.manhattan_distance(&end).cmp(&b.point.manhattan_distance(&end));
        let other = (a.point.manhattan_distance(&end) as i32 * a.k).cmp(&(b.point.manhattan_distance(&end) as i32 * b.k));

        k
    };

    let log = false;
    let mut most_k = i32::MAX;
    let mut most_visited: HashSet<Point> = HashSet::new();

    let mut visited = HashMap::new();
    let mut queue = VecDeque::from([
        Position {
            point: start.clone(),
            direction: Direction::Right,
            k: 0,
            visited: HashSet::from([start.clone()]),
        }
    ]);

    let mut counter = 0;
    while let Some(position) = queue.pop_front() {
        visited.insert(position.point, position.k);
        matrix[position.point.y as usize][position.point.x as usize] = 2;

        if position.point == end {
            match position.k.cmp(&most_k) {
                Ordering::Less => {
                    most_visited = position.visited.clone();
                    most_k = position.k;
                },
                Ordering::Equal => {
                    most_visited.extend(position.visited.clone());
                },
                Ordering::Greater => {}
            }
            continue;
        }

        [position.next(), position.right(), position.left()]
            .into_iter()
            .filter(|next| next.is_valid((limit_x, limit_y)))
            .filter(|next| matrix[next.point.y as usize][next.point.x as usize] != 1)
            .filter(|next| {
                if let Some(last) = visited.get(&next.point) { next.k <= *last }
                else { true }
            })
            .for_each(|next| queue.push_back(next));

        counter += 1;
        if log {
            std::process::Command::new("clear").status().unwrap();
            println!("[TOP = {most_k:?}] [SIZE = {}]", queue.len());
            println!("[{:?}] {:?} - {}", position.direction, position.point, position.k);
            matrix.iter().for_each(|line| println!("{}", line.iter().map(|i| match i {
                1 => "■",
                2 => "*",
                _ => " "
            }).join("")));
            std::thread::sleep(Duration::from_millis(500));
        }
    }

    most_visited.len() as i32
}

fn parse(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    match s {
                        "#" => 1,
                        _ => 0,
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect()
}