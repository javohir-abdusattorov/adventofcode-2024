use std::cmp::Ordering;
use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
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

pub fn part_2(input: String) -> i32 {
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