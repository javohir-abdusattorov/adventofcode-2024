#![allow(warnings)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::Read;
use std::time::Instant;
use std::{i128, iter, vec};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let (day, part) = read_day();
    let input = read_input(day, part);

    println!("Running for day #{day} part #{part}");

    let instant = Instant::now();
    let output = match (day, part) {
        (1, 1) => day_1_part_1(input),
        (1, 2) => day_1_part_2(input),

        (2, 1) => day_2_part_1(input),
        (2, 2) => day_2_part_2(input),

        (3, 1) => day_3_part_1(input),
        (3, 2) => day_3_part_2(input),

        (4, 1) => day_4_part_1(input),
        (4, 2) => day_4_part_2(input),

        (5, 1) => day_5_part_1(input),
        (5, 2) => day_5_part_2(input),

        (6, 1) => day_6_part_1(input),
        (6, 2) => day_6_part_2(input),

        (7, 1) => day_7_part_1(input) as i32,
        (7, 2) => day_7_part_2(input) as i32,

        (8, 1) => day_8_part_1(input),
        (8, 2) => day_8_part_2(input),

        (9, 1) => day_9_part_1(input) as i32,
        (9, 2) => day_9_part_2(input) as i32,
        _ => panic!("Solution not implemented for day #{day} part #{part}")
    };

    println!("Answer: {output}");
    println!("Time: {} ms", instant.elapsed().as_millis())
}

fn day_1_part_1(input: String) -> i32 {
    let (mut l, mut r): (BinaryHeap<i32>, BinaryHeap<i32>) = input
        .split("\n")
        .filter_map(|line| {
            line
                .split_whitespace()
                .filter_map(|str| str.parse::<i32>().ok())
                .next_tuple()
        })
        .unzip();

    iter::from_fn(|| l.pop())
        .zip(iter::from_fn(|| r.pop()))
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn day_1_part_2(input: String) -> i32 {
    let (l, r) = input
        .split("\n")
        .filter_map(|line| {
            line
                .split_whitespace()
                .filter_map(|str| str.parse::<i32>().ok())
                .next_tuple()
        })
        .fold((HashMap::new(), HashMap::new()), |mut set: (HashMap<i32, i32>, HashMap<i32, i32>), n: (i32, i32)| {
            *set.0.entry(n.0).or_insert(0) += n.0;
            *set.1.entry(n.1).or_insert(0) += 1;

            set
        });

    l
        .iter()
        .fold(0, |acc, (k, n)| acc + n * r.get(k).unwrap_or(&0))
}


fn day_2_part_1(input: String) -> i32 {
    input
        .split("\n")
        .filter(|line| {
            let line = line.split_whitespace().filter_map(|str| str.parse::<i32>().ok()).collect::<Vec<i32>>();
            let order = line[1] > line[0];

            line
                .iter()
                .enumerate()
                .skip(1)
                .all(|(i, _)| {
                    let diff = (line[i] - line[i - 1]).abs();
                    let current_order = line[i] > line[i - 1];

                    diff >= 1 && diff <= 3 && current_order == order
                })
        })
        .count() as i32
}

fn day_2_part_2(input: String) -> i32 {
    let is_safe = |line: &Vec<i32>| -> bool {
        let order = line[1] > line[0];

        line
            .iter()
            .enumerate()
            .skip(1)
            .all(|(i, _)| {
                let diff = (line[i] - line[i - 1]).abs();
                let current_order = line[i] > line[i - 1];

                diff >= 1 && diff <= 3 && current_order == order
            })
    };

    input
        .split("\n")
        .filter(|line| {
            let mut line = line
                .split_whitespace()
                .filter_map(|str| str.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            if is_safe(&line) {
                return true;
            }

            (0..line.len())
                .any(|i| {
                    let element = line.remove(i);
                    let is_safe_now = is_safe(&line);
                    line.insert(i, element);

                    is_safe_now
                })
        })
        .count() as i32
}


fn day_3_part_1(input: String) -> i32 {
    Regex
        ::new(r"mul\([0-9]+,[0-9]+\)")
        .unwrap()
        .find_iter(&input)
        .fold(0, |acc, str| {
            acc + str
                .as_str()
                .to_string()
                .replace(")", "")
                .replace("mul(", "")
                .split(",")
                .filter_map(|str| str.parse::<i32>().ok())
                .fold(1, |r, n| r * n)
        })
}

fn day_3_part_2(input: String) -> i32 {
    let mut can = true;
    Regex
        ::new(r"(mul\([0-9]+,[0-9]+\))|(don\'t\(\))|(do\(\))")
        .unwrap()
        .find_iter(&input)
        .fold(0, |acc, str| {
            let str = str.as_str();
            match str {
                "do()" => { can = true; acc },
                "don't()" => { can = false; acc },
                _ if can => acc + str
                    .replace(")", "")
                    .replace("mul(", "")
                    .split(",")
                    .filter_map(|str| str.parse::<i32>().ok())
                    .fold(1, |r, n| r * n),
                _ => { acc },
            }
        })
}


fn day_4_part_1(input: String) -> i32 {
    let mut matrix: Vec<Vec<&str>> = vec![];
    let mut x: Vec<(i32, i32)> = vec![];

    input
        .split("\n")
        .for_each(|line| {
            matrix.push(line
                .split("")
                .filter(|str| !str.is_empty())
                .enumerate()
                .map(|(i, str)| {
                    if str == "X" { x.push((matrix.len() as i32, i as i32)); }
                    str
                })
                .collect::<Vec<&str>>()
            );
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;
    let word = [ "M", "A", "S" ];
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    x.into_iter().fold(0, |total, (y, x)| {
        total + directions
            .iter()
            .filter(|direction| {
                let mut current_y = y;
                let mut current_x = x;

                word.iter().all(|char| {
                    current_y += direction.0;
                    current_x += direction.1;

                    current_y >= 0 && current_x >= 0 && current_y <= limit_y && current_x <= limit_x && &matrix[current_y as usize][current_x as usize] == char
                })
            })
            .count()
    }) as i32
}

fn day_4_part_2(input: String) -> i32 {
    let mut matrix: Vec<Vec<&str>> = vec![];
    let mut a: Vec<(i32, i32)> = vec![];

    input
        .split("\n")
        .for_each(|line| {
            matrix.push(line
                .split("")
                .filter(|str| !str.is_empty())
                .enumerate()
                .map(|(i, str)| {
                    if str == "A" { a.push((matrix.len() as i32, i as i32)); }
                    str
                })
                .collect::<Vec<&str>>()
            );
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;
    let directions = [
        [(-1, -1), (1, 1)],
        [(1, -1), (-1, 1)],
    ];

    a
        .into_iter()
        .filter(|(y, x)| directions.iter().all(|direction| {
            let strs = direction
                .iter()
                .filter_map(|direction| {
                    let y = y + direction.0;
                    let x = x + direction.1;

                    if y >= 0 && x >= 0 && y <= limit_y && x <= limit_x && (matrix[y as usize][x as usize] == "M" || matrix[y as usize][x as usize] == "S") {
                        Some(matrix[y as usize][x as usize])
                    } else {
                        None
                    }
                })
                .collect::<Vec<&str>>();

            strs.len() == 2 && strs.contains(&"M") && strs.contains(&"S")
        }))
        .count() as i32
}


fn day_5_part_1(input: String) -> i32 {
    let mut parts = input.split("\n\n");
    let m = parts
        .next()
        .unwrap()
        .split("\n")
        .fold(HashMap::new(), |mut m, line| {
            let (a, b) = line.split("|").filter_map(|str| str.parse::<i32>().ok()).next_tuple().unwrap();
            m.entry(a).or_insert(Vec::new()).push(b);

            m
        });

    parts
        .next()
        .unwrap()
        .split("\n")
        .fold(0, |n, line| {
            let numbers = line.split(",").filter_map(|str| str.parse::<i32>().ok()).collect::<Vec<i32>>();
            for i in 1..numbers.len() {
                if let Some(ordering) = m.get(&numbers[i - 1]) {
                    if ordering.contains(&numbers[i]) {
                        continue;
                    }
                }

                return n;
            }

            n + numbers[numbers.len() / 2]
        })
}

fn day_5_part_2(input: String) -> i32 {
    let mut parts = input.split("\n\n");
    let m = parts
        .next()
        .unwrap()
        .split("\n")
        .fold(HashMap::new(), |mut m, line| {
            let (a, b) = line.split("|").filter_map(|str| str.parse::<i32>().ok()).next_tuple().unwrap();
            m.entry(a).or_insert(Vec::new()).push(b);

            m
        });

    parts
        .next()
        .unwrap()
        .split("\n")
        .map(|line| line.split(",").filter_map(|str| str.parse::<i32>().ok()).collect::<Vec<i32>>())
        .filter(|numbers| {
            !numbers
                .iter()
                .zip(numbers.iter().skip(1))
                .all(|(prev, current)| {
                    let ordering = m.get(&prev);
                    ordering.is_some() && ordering.unwrap().contains(current)
                })
        })
        .map(|mut numbers| {
            numbers.sort_by(|a, b| {
                let a_ordering = m.get(a);
                let b_ordering = m.get(b);

                if let Some(a) = a_ordering {
                    if a.contains(b) {
                        return Ordering::Greater;
                    }
                }

                if let Some(b) = b_ordering {
                    if b.contains(a) {
                        return Ordering::Less;
                    }
                }

                std::cmp::Ordering::Equal
            });

            numbers
        })
        .fold(0, |n, numbers| n + numbers[numbers.len() / 2])
}


fn day_6_part_1(input: String) -> i32 {
    #[derive(Debug)]
    enum Direction {
        UP = 1,
        DOWN = -1,
        RIGHT = 2,
        LEFT = -2
    }

    impl Direction {
        fn rotate(&mut self) {
            *self = match self {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            }
        }

        fn next(&self) -> (i32, i32) {
            match self {
                Direction::UP => (-1, 0),
                Direction::DOWN => (1, 0),
                Direction::RIGHT => (0, 1),
                Direction::LEFT => (0, -1),
            }
        }
    }

    let mut guard = (-1, -1);
    let mut matrix: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let mut line = line
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>();

            if guard.0 == -1 && guard.1 == -1 {
                if let Some((x, _)) = line.iter().find_position(|s| **s == '^') {
                    guard = (y as i32, x as i32);
                    line[x] = 'X';
                }
            }

            matrix.push(line);
            matrix
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;

    let mut direction = Direction::UP;
    let mut n = 1;

    loop {
        let step = direction.next();
        let next = (guard.0 + step.0, guard.1 + step.1);

        if next.1 < 0 || next.0 < 0 || next.1 > limit_x || next.0 > limit_y {
            break;
        }

        match matrix[next.0 as usize][next.1 as usize] {
            '#' => {
                direction.rotate();
            },
            'X' => {
                guard = next;
            },
            '.' => {
                guard = next;
                matrix[next.0 as usize][next.1 as usize] = 'X';
                n += 1;
            },
            _ => {}
        };
    }

    n
}

fn day_6_part_2(input: String) -> i32 {
    type Position = (i32, i32);

    #[derive(Debug, Clone, Copy)]
    enum Direction {
        UP = 1,
        DOWN = -1,
        RIGHT = 2,
        LEFT = -2
    }

    impl Direction {
        fn rotate(&mut self) {
            *self = match self {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            }
        }

        fn next(&self) -> Position {
            match self {
                Direction::UP => (-1, 0),
                Direction::DOWN => (1, 0),
                Direction::RIGHT => (0, 1),
                Direction::LEFT => (0, -1),
            }
        }

        fn prev(&self) -> Position {
            match self {
                Direction::UP => (1, 0),
                Direction::DOWN => (-1, 0),
                Direction::RIGHT => (0, -1),
                Direction::LEFT => (0, 1),
            }
        }

        fn close(&self) -> (Ordering, Ordering) {
            match self {
                Direction::UP => (Ordering::Equal, Ordering::Greater),
                Direction::DOWN => (Ordering::Equal, Ordering::Less),
                Direction::RIGHT => (Ordering::Greater, Ordering::Equal),
                Direction::LEFT => (Ordering::Less, Ordering::Equal),
            }
        }

        fn near(&self) -> (Ordering, Ordering) {
            match self {
                Direction::UP => (Ordering::Equal, Ordering::Less),
                Direction::DOWN => (Ordering::Equal, Ordering::Greater),
                Direction::RIGHT => (Ordering::Less, Ordering::Equal),
                Direction::LEFT => (Ordering::Greater, Ordering::Equal),
            }
        }
    }

    let mut guard = (-1, -1);
    let mut obstacles: Vec<Position> = Vec::new();

    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line_str)| {
            let mut line = vec![];

            line_str
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .enumerate()
                .for_each(|(x, mut char)| {
                    match char {
                        '#' => obstacles.push((y as i32, x as i32)),
                        '^' if guard.0 == -1 && guard.1 == -1 => {
                            guard = (y as i32, x as i32);
                            char = '.';
                        },
                        _ => {},
                    }

                    line.push(char);
                });

            matrix.push(line);
            matrix
        });

    let limit_y = matrix.len() as i32 - 1;
    let limit_x = matrix[0].len() as i32 - 1;

    let mut direction = Direction::UP;
    let mut n = 0;

    let nearest_obstacle = |obstacles: &Vec<Position>, direction: &Direction, guard: &Position| -> Option<Position> {
        let compare_find = direction.close();
        let compore_sort = direction.near();

        obstacles
            .iter()
            .filter(|obstacle| {
                obstacle.0.cmp(&guard.0).eq(&compare_find.0) && 
                obstacle.1.cmp(&guard.1).eq(&compare_find.1) &&
                (obstacle.0.abs_diff(guard.0) > 1 ||
                obstacle.1.abs_diff(guard.1) > 1)
            })
            .sorted_by(|a, b| {
                if if compore_sort.0 != Ordering::Equal {
                    a.0.cmp(&b.0).eq(&compore_sort.0)
                } else {
                    a.1.cmp(&b.1).eq(&compore_sort.1)
                } {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .next()
            .map(|v| *v)
    };

    loop {
        let step = direction.next();
        let next = (guard.0 + step.0, guard.1 + step.1);

        if next.1 < 0 || next.0 < 0 || next.1 > limit_x || next.0 > limit_y {
            break;
        }

        if matrix[next.0 as usize][next.1 as usize] == '#' {
            direction.rotate();
            continue;
        }

        let near_obstacle = nearest_obstacle(&obstacles, &direction, &guard);
        if near_obstacle.is_none() {
            guard = next;
            continue;
        }

        let mut direction = direction.clone();
        let mut near_obstacle = near_obstacle.unwrap();
        let mut obstacled_guard = guard.clone();

        direction.rotate();
        obstacles.push(next);

        loop {
            let prev = direction.prev();

            obstacled_guard.0 = near_obstacle.0 + prev.0;
            obstacled_guard.1 = near_obstacle.1 + prev.1;

            if obstacled_guard.0 == guard.0 && obstacled_guard.1 == guard.1 {
                n += 1;
                break;
            }

            near_obstacle = match nearest_obstacle(&obstacles, &direction, &obstacled_guard) {
                None => break,
                Some(v) => v,
            };

            direction.rotate();
        }

        obstacles.pop();
        guard = next;
    }

    n
}


fn day_7_part_1(input: String) -> i128 {
    #[derive(Debug)]
    enum Operators { Add, Multiply }

    impl Operators {
        fn eval(&self, a: i128, b: i128) -> i128 {
            match self {
                Operators::Add => a + b,
                Operators::Multiply => a * b,
            }
        }
    }

    fn cmb(prev: i128, nums: &Vec<i128>, i: usize, target: i128) -> bool {
        if i >= nums.len() {
            return prev == target;
        }

        [Operators::Add, Operators::Multiply]
            .into_iter()
            .any(|operator| {
                let ev = operator.eval(prev, nums[i]);
                cmb(ev, nums, i + 1, target)
            })
    }

    input
        .split("\n")
        .map(|line| {
            let mut parts = line.split(": ");
            let eq = parts.next().unwrap().parse::<i128>().unwrap();
            let nums = parts.next().unwrap().split_whitespace().filter_map(|n| n.parse::<i128>().ok()).collect::<Vec<i128>>();

            (eq, nums)
        })
        .filter(|(target, nums)| cmb(nums[0], nums, 1, *target))
        .fold(0, |acc, (target, _)| acc + target)
}

fn day_7_part_2(input: String) -> i128 {
    #[derive(Debug)]
    enum Operators { Add, Multiply, Concatenate }

    impl Operators {
        fn eval(&self, a: i128, b: i128) -> i128 {
            match self {
                Operators::Add => a + b,
                Operators::Multiply => a * b,
                Operators::Concatenate => format!("{}{}", a.to_string(), b.to_string()).parse::<i128>().unwrap()
            }
        }
    }

    fn cmb(prev: i128, nums: &Vec<i128>, i: usize, target: i128) -> bool {
        if i >= nums.len() {
            return prev == target;
        }

        [Operators::Add, Operators::Multiply, Operators::Concatenate]
            .into_iter()
            .any(|operator| {
                let ev = operator.eval(prev, nums[i]);
                cmb(ev, nums, i + 1, target)
            })
    }

    input
        .split("\n")
        .map(|line| {
            let mut parts = line.split(": ");
            let eq = parts.next().unwrap().parse::<i128>().unwrap();
            let nums = parts.next().unwrap().split_whitespace().filter_map(|n| n.parse::<i128>().ok()).collect::<Vec<i128>>();

            (eq, nums)
        })
        .filter(|(target, nums)| cmb(nums[0], nums, 1, *target))
        .fold(0, |acc, (target, _)| acc + target)
}


fn day_8_part_1(input: String) -> i32 {
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

fn day_8_part_2(input: String) -> i32 {
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


fn day_9_part_1(input: String) -> i128 {
    let mut r = 0;
    let mut v = input
        .split("")
        .filter_map(|str| str.parse::<usize>().ok())
        .enumerate()
        .fold((vec![], 0), |(mut acc, p), (index, n)| {
            if index % 2 != 0 {
                acc.append(&mut vec![-1; n]);
                (acc, p)
            } else {
                acc.append(&mut vec![p; n]);
                r = acc.len() - 1;
                (acc, p + 1)
            }
        })
        .0;

    for i in 0..v.len() {
        if v[i] != -1 { continue; }
        if r <= i { break; }

        v.swap(i, r);
        while v[r] == -1 { r -= 1 }
    }

    v.into_iter()
        .enumerate()
        .filter(|(_, n)| *n != -1)
        .fold(0, |acc: i128, (i, n)| acc + (n as i128 * i as i128))
}

fn day_9_part_2(input: String) -> i128 {
    let (mut mem, mut file_slots, mut open_slots, _) = input
        .split("")
        .filter_map(|str| str.parse::<usize>().ok())
        .enumerate()
        .fold((vec![], vec![], vec![], 0), |(mut acc, mut file_slots, mut open_slots, p), (index, n)| {
            if index % 2 != 0 {
                open_slots.push((acc.len(), n));
                acc.append(&mut vec![-1; n]);
                (acc, file_slots, open_slots, p)
            }
            else {
                file_slots.push((acc.len(), n));
                acc.append(&mut vec![p; n]);
                (acc, file_slots, open_slots, p + 1)
            }
        });

    for (i, size) in file_slots.into_iter().skip(1).rev() {
        if let Some((pos, open_slot)) = open_slots.iter().filter(|slot| slot.0 < i).find_position(|slot| slot.1 >= size) {
            (0..size).for_each(|k| mem.swap(k + open_slot.0, k + i));

            open_slots[pos] = (open_slots[pos].0 + size, open_slots[pos].1 - size);
            if open_slots[pos].1 <= 0 {
                open_slots.remove(pos);
            }
        }
    }

    mem.into_iter()
        .enumerate()
        .filter(|(_, n)| *n != -1)
        .fold(0, |acc: i128, (i, n)| acc + (n as i128 * i as i128))
}



fn read_day() -> (i32, i32) {
    std::env::args()
        .skip(1)
        .take(1)
        .collect::<String>()
        .split("-")
        .filter_map(|str| str.parse::<i32>().ok())
        .next_tuple()
        .expect("Input arguments should be DAY-PART, for example: 1-2")
}

fn read_input(day: i32, part: i32) -> String {
    let mut input: String = String::new();
    let input_path = format!("inputs/{day}-{part}.txt");

    std::fs::File::open(input_path.clone())
        .expect(&format!("Could't open {} file", input_path))
        .read_to_string(&mut input)
        .expect(&format!("Could't read file: {}", input_path));

    input
}