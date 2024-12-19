use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
    let mut matrix: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let mut line = line
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>();

            matrix.push(line);
            matrix
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;
    let sides = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    let bfs = |p: (i32, i32), matrix: &mut Vec<Vec<char>>| -> i32 {
        let ch = matrix[p.0 as usize][p.1 as usize];
        let mut visited = HashSet::from([]);
        let mut queue = VecDeque::from([p]);

        let mut area = 0;
        let mut perimeter = 0;

        while let Some(p) = queue.pop_front() {
            if visited.contains(&p) { continue; }

            matrix[p.0 as usize][p.1 as usize] = '#';
            visited.insert(p);
            area += 1;
            perimeter += 4;

            sides
                .iter()
                .filter_map(|side| {
                    let next = (p.0 + side.0, p.1 + side.1);
                    (next.0 >= 0 && next.1 >= 0 && next.0 <= limit_y && next.1 <= limit_x && [ch, '#'].contains(&matrix[next.0 as usize][next.1 as usize])).then_some(next)
                })
                .filter(|p| {
                    if visited.contains(p) || matrix[p.0 as usize][p.1 as usize] == ch {
                        perimeter -= 1;
                    }

                    matrix[p.0 as usize][p.1 as usize] == ch
                })
                .for_each(|p| {
                    queue.push_back(p);
                });
        }

        (area * perimeter) as i32
    };

    let mut k = 0;
    for y in 0..=limit_y {
        for x in 0..=limit_x {
            if matrix[y as usize][x as usize] != '#' {
                k += bfs((y, x), &mut matrix);
            }
        }
    }

    k
}

pub fn part_2(input: String) -> i32 {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    enum Directions {
        Up, Down, Left, Right
    }

    let inner_directions = [
        (Directions::Up, Directions::Right),
        (Directions::Up, Directions::Left),
        (Directions::Down, Directions::Right),
        (Directions::Down, Directions::Left),
    ];

    let mut matrix: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let mut line = line
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>();

            matrix.push(line);
            matrix
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;
    let sides = [
        (1, 0, Directions::Up),
        (-1, 0, Directions::Down),
        (0, 1, Directions::Right),
        (0, -1, Directions::Left),
    ];

    let is_valid = |p: (i32, i32)| -> bool {
        (p.0 >= 0 && p.1 >= 0 && p.0 <= limit_y && p.1 <= limit_x)
    };

    let sides_of = |p: (i32, i32), ch: char, matrix: &Vec<Vec<char>>| -> i32 {
        let map: HashMap<Directions, (i32, i32)> = sides
            .iter()
            .filter_map(|side| {
                let next = (p.0 + side.0, p.1 + side.1);
                (is_valid(next) && matrix[next.0 as usize][next.1 as usize] == ch).then_some((next.0, next.1, side.2))
            })
            .map(|(y, x, d)| (d, (y, x)))
            .collect::<HashMap<Directions, (i32, i32)>>();

        let outer = match map.len() {
            0 => 4,
            1 => 2,
            2 => {
                let ((_, a), (_, b)) = map.iter().next_tuple().unwrap();
                if a.0 == b.0 || a.1 == b.1 { 0 } else { 1 }
            }
            _ => 0,
        };

        let inner = inner_directions
            .iter()
            .map(|(a, b)| (map.get(a), map.get(b)))
            .filter_map(|(a, b)| a.zip(b))
            .map(|(a, b)| (a.0, b.1))
            .filter(|next| is_valid(*next) && matrix[next.0 as usize][next.1 as usize] != ch)
            .count() as i32;

        outer + inner
    };

    let bfs = |p: (i32, i32), matrix: &mut Vec<Vec<char>>| -> i32 {
        let ch = matrix[p.0 as usize][p.1 as usize];
        let mut visited = HashSet::from([]);
        let mut queue = VecDeque::from([p]);

        let mut area = 0;
        let mut perimeter = 0;

        while let Some(p) = queue.pop_front() {
            if visited.contains(&p) { continue; }

            visited.insert(p);
            area += 1;
            perimeter += sides_of(p, ch, matrix);

            sides
                .iter()
                .filter_map(|side| {
                    let next = (p.0 + side.0, p.1 + side.1);
                    (is_valid(next) && matrix[next.0 as usize][next.1 as usize] == ch).then_some(next)
                })
                .for_each(|p| queue.push_back(p));
        }

        visited.iter().for_each(|(y, x)| matrix[*y as usize][*x as usize] = '#');
        (area * perimeter) as i32
    };

    let mut k = 0;
    for y in 0..=limit_y {
        for x in 0..=limit_x {
            if matrix[y as usize][x as usize] != '#' {
                let v = bfs((y, x), &mut matrix);
                k += v;
            }
        }
    }

    k
}