use std::{cmp::Ordering, collections::{HashSet, VecDeque}};
use itertools::Itertools;

use crate::util::point::Point;


pub fn part_1( input: String ) -> i32 {
    const CHEATS: i32 = 2;
    const SAVE: i32 = 100;

    let (start, end, mut matrix) = parse(input);

    dfs(&mut matrix, start, end)
        .into_iter()
        .map(|point| (matrix[point.y as usize][point.x as usize], point))
        .map(|(current, point)| {
            [Point::up(), Point::down(), Point::right(), Point::left()]
                .into_iter()
                .map(|direction| (direction, point + direction))
                .filter(|(direction, next)| is_valid(&next, &matrix, -1, Ordering::Equal))
                .map(|(direction, next)| next + direction)
                .filter(|next| is_valid(&next, &matrix, -1, Ordering::Greater))
                .map(|next| matrix[next.y as usize][next.x as usize])
                .filter(|next| *next > current)
                .map(|score| score - current - CHEATS)
                .filter(|score| *score >= SAVE)
                .count() as i32
        })
        .sum()
}

pub fn part_2( input: String ) -> i32 {
    const CHEATS: i32 = 20;
    const SAVE: i32 = 100;

    let (start, end, mut matrix) = parse(input);
    let area = area(CHEATS);

    dfs(&mut matrix, start, end)
        .into_iter()
        .map(|point| (matrix[point.y as usize][point.x as usize], point))
        .map(|(current, point)| {
            area
                .clone()
                .into_iter()
                .map(|direction| (direction, point + direction))
                .filter(|(direction, next)| is_valid(&next, &matrix, -1, Ordering::Greater))
                .map(|(direction, next)| (matrix[next.y as usize][next.x as usize], next.manhattan_distance(&point) as i32))
                .filter(|(score, distance)| *score > current)
                .map(|(score, distance)| score - current - distance)
                .filter(|score| *score >= SAVE)
                .count() as i32
        })
        .sum()
}

fn dfs(matrix: &mut Vec<Vec<i32>>, start: Point, end: Point) -> Vec<Point> {
    let mut queue = VecDeque::from([ (start, 0) ]);
    let mut path = Vec::new();

    while let Some((point, k)) = queue.pop_back() {
        path.push(point);
        matrix[point.y as usize][point.x as usize] = k;

        if point == end { break; }

        [Point::up(), Point::down(), Point::right(), Point::left()]
            .into_iter()
            .map(|direciton| point + direciton)
            .filter(|next| is_valid(next, &matrix, 1, Ordering::Equal))
            .for_each(|next| queue.push_back((next, k + 1)))
    }

    path
}

fn is_valid(point: &Point, matrix: &Vec<Vec<i32>>, eq: i32, order: Ordering) -> bool {
    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;

    point.x >= 0 && point.y >= 0 && point.x < limit_x && point.y < limit_y && matrix[point.y as usize][point.x as usize].cmp(&eq) == order
}

fn area(r: i32) -> HashSet<Point> {
    let mut side = HashSet::new();

    (0..=r).for_each(|y| (0..=r-y).for_each(|x| { side.insert(Point::new(x, y)); }));
    (0..=r).for_each(|x| (0..=r-x).for_each(|y| { side.insert(Point::new(x, y)); }));

    HashSet::from_iter(
        side.clone().into_iter()
        .chain(side.clone().into_iter().map(|point| Point::new(-point.x, point.y)))
        .chain(side.clone().into_iter().map(|point| Point::new(point.x, -point.y)))
        .chain(side.clone().into_iter().map(|point| Point::new(-point.x, -point.y)))
    )
}

fn parse(input: String) -> (Point, Point, Vec<Vec<i32>>) {
    let mut s = Point::new(-1, -1);
    let mut e = Point::new(-1, -1);

    let mut matrix = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .split("")
                .filter(|s| !s.is_empty())
                .enumerate()
                .map(|(x, char)| {
                    match char {
                        "#" => -1,
                        "S" => { s = Point::new(x as i32, y as i32); 1 },
                        "E" => { e = Point::new(x as i32, y as i32); 1 },
                        _ => 1,
                    }
                })
                .collect::<Vec<i32>>()
        })
        .collect();

    (s, e, matrix)
}