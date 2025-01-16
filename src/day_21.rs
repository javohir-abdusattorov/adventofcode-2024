use std::collections::{HashMap, VecDeque};
use std::f32::consts::E;
use std::thread::panicking;
use std::u128;
use itertools::Itertools;

use crate::util::point::{self, Point};
use crate::day_16::Direction;


#[derive(Debug)]
struct Position {
    k: i32,
    point: Point,
    direction: Direction,
    visited: Vec<Direction>,
}

impl Position {
    fn is_valid(&self, limit: Point) -> bool {
        self.point.x >= 0 && self.point.y >= 0 && self.point.x <= limit.x && self.point.y <= limit.y
    }

    fn next(&self) -> Position {
        self.go(self.direction, 1)
    }

    fn right(&self) -> Position {
        self.go(self.direction.right(), 101)
    }

    fn left(&self) -> Position {
        self.go(self.direction.left(), 101)
    }

    fn back(&self) -> Position {
        self.go(self.direction.back(), 101)
    }

    fn go(&self, direction: Direction, k: i32) -> Position {
        let point = direction.next(self.point);
        let mut visited = self.visited.clone();
        visited.push(direction);

        Position {
            point,
            direction,
            visited,
            k: self.k + k,
        }
    }
}


pub fn part_1(input: String) -> i32 {
    const ROBOTS: u8 = 2;

    parse(input)
        .into_iter()
        .map(|code| (code.clone(), code[0..3].parse::<i32>().unwrap()))
        .map(|(code, numeric)| {
            let types = bfs_typer(code, ROBOTS);
            types.len() as i32 * numeric
        })
        .sum()
}

pub fn part_2(input: String) -> u128 {
    const ROBOTS: u8 = 25;
    let mut memoize = HashMap::new();

    parse(input)
        .into_iter()
        .map(|code| (code.clone(), code[0..3].parse::<u128>().unwrap()))
        .map(|(code, numeric)| {
            let count = dfs_typer(code.clone(), ROBOTS, &mut memoize);
            count * numeric
        })
        .sum()
}


fn bfs_typer(code: String, stages: u8) -> String {
    let (number_board, robot_board) = keyboards();
    let (number_indexes, number_cache) = board_info(&number_board);
    let (robot_indexes, robot_cache) = board_info(&robot_board);

    let mut final_stage = single_level(
        Point::new(2, 3),
        code,
        &number_board,
        &number_indexes,
        &number_cache,
    );

    for i in 0..stages {
        final_stage = single_level(
            Point::new(2, 0),
            final_stage,
            &robot_board,
            &robot_indexes,
            &robot_cache,
        );
    }

    final_stage
}

fn dfs_typer(code: String, stages: u8, memoize: &mut HashMap<(String, u8), u128>) -> u128 {
    let (number_board, robot_board) = keyboards();
    let (number_indexes, number_cache) = board_info(&number_board);
    let (robot_indexes, robot_cache) = board_info(&robot_board);

    let mut final_stage = single_level(
        Point::new(2, 3),
        code,
        &number_board,
        &number_indexes,
        &number_cache,
    );

    multi_level(
        stages,
        final_stage,
        &robot_board,
        &robot_indexes,
        &robot_cache,
        memoize,
    )
}

fn shortest_path(matrix: Vec<Vec<char>>) -> HashMap<(Point, Point), Vec<Direction>> {
    let indexes = char_indexes(&matrix);
    let keys = matrix.clone().into_iter().flatten().collect::<Vec<char>>();
    let find_others = |key: &char, list: &Vec<char>| -> Vec<char> {
        list.iter().filter(|i| *i != key && **i != '#').map(|c| c.clone()).collect::<Vec<char>>()
    };

    let mut cache = HashMap::new();

    for start in keys.iter().filter(|char| **char != '#') {
        let others = find_others(start, &keys);
        for end in others {
            let start_point = indexes.get(&start).unwrap().clone();
            let end_point = indexes.get(&end).unwrap().clone();
            let paths = search(matrix.clone(), start_point, end_point);
            cache.insert((start_point, end_point), paths);
        }
    }

    cache
}

fn single_level(mut start: Point, target: String, matrix: &Vec<Vec<char>>, indexes: &HashMap<char, Point>, cache: &HashMap<(Point, Point), Vec<Direction>>) -> String {
    target
        .chars()
        .map(|char| {
            let end = indexes.get(&char).expect(format!("{char} does not exist in map").as_str()).clone();
            if start == end { return vec!['A'] }

            let search = cache.get(&(start, end)).unwrap();
            let mut moves = search.into_iter().map(|direction| direction.print()).collect::<Vec<char>>();

            moves.push('A');
            start = end;
 
            moves
        })
        .flatten()
        .join("")
}

fn multi_level(level: u8, target: String, matrix: &Vec<Vec<char>>, indexes: &HashMap<char, Point>, paths: &HashMap<(Point, Point), Vec<Direction>>, memoize: &mut HashMap<(String, u8), u128>) -> u128 {
    if let Some(precomputed) = memoize.get(&(target.clone(), level)) {
        return *precomputed;
    }

    if level < 1 {
        return target.len() as u128
    }

    let iterator = ['A'].into_iter().chain(target.chars());
    iterator
        .clone()
        .zip(iterator.skip(1))
        .map(|(start, end)| {
            let mut moves = String::from("A");
            if start != end {
                let start = indexes.get(&start).unwrap().clone();
                let end = indexes.get(&end).unwrap().clone();

                moves = paths
                    .get(&(start, end))
                    .unwrap()
                    .into_iter()
                    .map(|direction| direction.print())
                    .chain(['A'])
                    .collect()
            }

            let count = multi_level(level - 1, moves.clone(), matrix, indexes, paths, memoize);
            let key = (moves, level - 1);
            memoize.entry(key).or_insert(count);

            count
        })
        .sum()
}

fn search(matrix: Vec<Vec<char>>, start: Point, end: Point) -> Vec<Direction> {
    let x_diff = start.x.abs_diff(end.x);
    let y_diff = start.y.abs_diff(end.y);

    let x_direction = if start.x > end.x { Direction::Left } else { Direction::Right };
    let y_direction = if start.y > end.y { Direction::Up } else { Direction::Down };

    let mut x_moves = vec![x_direction; x_diff as usize];
    let mut y_moves = vec![y_direction; y_diff as usize];

    if x_diff == 0 {
        return y_moves;
    }
    else if y_diff == 0 {
        return x_moves;
    }

    let x_then_y = {
        let mut x_moves = x_moves.clone();
        x_moves.append(&mut y_moves.clone());
        x_moves
    };

    let y_then_x = {
        let mut y_moves = y_moves.clone();
        y_moves.append(&mut x_moves.clone());
        y_moves
    };

    let order = if x_direction == Direction::Left {
        (x_then_y, y_then_x)
    } else {
        (y_then_x, x_then_y)
    };

    if walls_exist(&matrix, start, order.0.clone()) {
        order.1
    } else {
        order.0
    }
}

fn walls_exist(matrix: &Vec<Vec<char>>, mut point: Point, moves: Vec<Direction>) -> bool {
    moves
        .into_iter()
        .any(|direction| {
            point = direction.next(point);
            matrix[point.y as usize][point.x as usize] == '#'
        })
}

fn char_indexes(matrix: &Vec<Vec<char>>) -> HashMap<char, Point> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .map(move |(x, char)| (char.clone(), Point::new(x as i32, y as i32)))
        })
        .collect()
}

fn parse(input: String) -> Vec<String> {
    input
        .lines()
        .map(|str| str.to_string())
        .collect()
}

fn keyboards() -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    (
        vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec!['#', '0', 'A'],
        ],
        vec![
            vec!['#', '^', 'A'],
            vec!['<', 'v', '>'],
        ]
    )
}

fn board_info(matrix: &Vec<Vec<char>>) -> (HashMap<char, Point>, HashMap<(Point, Point), Vec<Direction>>) {
    (
        char_indexes(matrix),
        shortest_path(matrix.clone()),
    )   
}