use std::{cmp::Ordering, collections::VecDeque};
use itertools::Itertools;

use crate::util::point::Point;


pub fn part_1( input: String ) -> i32 {
    const N: i32 = 71;
    const BYTES: usize = 1024;

    let mut matrix = (0..N).map(|_| vec![0; N as usize]).collect::<Vec<Vec<u8>>>();

    input
        .lines()
        .take(BYTES)
        .map(|line| line
            .split(",")
            .filter_map(|str| str.parse::<usize>().ok())
            .next_tuple::<(usize, usize)>()
            .unwrap()
        )
        .for_each(|(x, y)| matrix[y][x] = 1);

    min_distance(matrix)
}

pub fn part_2( input: String ) -> i32 {
    const N: i32 = 71;
    const BYTES: usize = 1024;

    let mut matrix = (0..N).map(|_| vec![0; N as usize]).collect::<Vec<Vec<u8>>>();
    let mut bytes = input
        .lines()
        .map(|line| line
            .split(",")
            .filter_map(|str| str.parse::<i32>().ok())
            .next_tuple::<(i32, i32)>()
            .unwrap()
        )
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<Point>>();

    bytes
        .iter()
        .take(BYTES)
        .for_each(|point| matrix[point.y as usize][point.x as usize] = 1);

    for point in bytes.into_iter().skip(BYTES) {
        matrix[point.y as usize][point.x as usize] = 1;
        if min_distance(matrix.clone()) == -1 {
            println!("point = {point:?}");
            break;
        }
    }

    0
}

fn min_distance(mut matrix: Vec<Vec<u8>>) -> i32 {
    const N: i32 = 71;
    const SIZE: usize = 50;

    let start = Point::new(0, 0);
    let end = Point::new(N - 1, N - 1);
    let directions = [
        Point::up(),
        Point::down(),
        Point::right(),
        Point::left(),
    ];

    let mut k = 0;
    let mut queue_2 = Vec::new();
    let mut queue_1 = Vec::from([start]);

    let is_valid = |point: &Point| -> bool {
        point.x >= 0 && point.y >= 0 && point.x < N && point.y < N
    };

    let sorter = |a: &Point, b: &Point| -> Ordering {
        let manhattan = a.manhattan_distance(&end).cmp(&b.manhattan_distance(&end));
        manhattan
    };

    while let point = queue_1.remove(0) {
        if point == end {
            break;
        }

        matrix[point.y as usize][point.x as usize] = 2;

        directions
            .iter()
            .map(|next| point + *next)
            .filter(is_valid)
            .filter(|next| matrix[next.y as usize][next.x as usize] == 0)
            .for_each(|next| {
                if !queue_2.contains(&next) {
                    queue_2.push(next.clone())
                }
            });

        if queue_1.is_empty() && queue_2.is_empty() {
            return -1;
        }

        if queue_1.is_empty() {
            k += 1;
            queue_2.sort_by(sorter);
            queue_2.drain(queue_2.len().min(SIZE)..);
            std::mem::swap(&mut queue_1, &mut queue_2);
        }
    }

    k
}