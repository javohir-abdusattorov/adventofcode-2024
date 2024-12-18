use std::collections::HashMap;


pub fn part_1(input: String) -> i32 {
    let mut freq = HashMap::new();

    let mut matrix: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let line = line
                .split("")
                .filter(|s| !s.is_empty())
                .enumerate()
                .map(|(x, s)| {
                    let c = s.chars().next().unwrap();
                    if c != '.' {
                        freq.entry(c).or_insert(Vec::new()).push((y as i32, x as i32));
                    }

                    c
                })
                .collect::<Vec<char>>();

            matrix.push(line);
            matrix
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;

    let next = |a: i32, b: i32| -> i32 {
        let abs = a.abs_diff(b) as i32;
        if a < b {
            b + abs
        } else {
            b - abs
        }
    };

    freq.iter().fold(0, |acc, (_, locations)| {
        acc + locations.iter().fold(0, |acc, location| {
            acc + locations
                .iter()
                .filter(|v| *v != location)
                .map(|other| (next(location.0, other.0), next(location.1, other.1)))
                .filter(|(y, x)| *x >= 0 && *y >= 0 && *x <= limit_x && *y <= limit_y)
                .fold(0, |acc, (y, x)| {
                    if matrix[y as usize][x as usize] == '#' {
                        return acc;
                    }
    
                    matrix[y as usize][x as usize] = '#';
                    acc + 1
                })
        })
    })
}

pub fn part_2(input: String) -> i32 {
    let mut freq = HashMap::new();

    let mut matrix: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let line = line
                .split("")
                .filter(|s| !s.is_empty())
                .enumerate()
                .map(|(x, s)| {
                    let c = s.chars().next().unwrap();
                    if c != '.' {
                        freq.entry(c).or_insert(Vec::new()).push((y as i32, x as i32));
                    }

                    c
                })
                .collect::<Vec<char>>();

            matrix.push(line);
            matrix
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;

    let next = |a: i32, b: i32| -> i32 {
        let abs = a.abs_diff(b) as i32;
        if a < b { b + abs } else { b - abs }
    };

    freq.iter().fold(0, |acc, (_, locations)| {
        acc + locations.iter().fold(0, |acc, location| {
            acc + locations
                .iter()
                .filter(|v| *v != location)
                .fold(0, |acc, other| {
                    let mut from = location.clone();
                    let mut to = other.clone();
                    let mut sum = 0;

                    loop {
                        if matrix[to.0 as usize][to.1 as usize] != '#' {
                            matrix[to.0 as usize][to.1 as usize] = '#';
                            sum += 1;
                        }

                        let (y, x) = (next(from.0, to.0), next(from.1, to.1));
                        if y < 0 || x < 0 || y > limit_y || x > limit_x {
                            break;
                        }

                        from = to;
                        to = (y, x);
                    }

                    acc + sum
                })
        })
    })
}