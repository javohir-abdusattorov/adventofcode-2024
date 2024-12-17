use std::collections::HashSet;


type Position = (i32, i32);

pub fn part_1(input: String) -> i32 {
    let mut heads: Vec<Position> = vec![];
    let mut matrix: Vec<Vec<u8>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let line = line
                .split("")
                .filter_map(|str| str.parse::<u8>().ok())
                .enumerate()
                .map(|(x, n)| {
                    if n == 0 { heads.push((y as i32, x as i32)) }
                    n
                })
                .collect::<Vec<u8>>();

            matrix.push(line);
            matrix
        });

    fn trail(pos: Position, n: u8, matrix: &Vec<Vec<u8>>, visited: &mut HashSet<Position>) -> i32 {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= matrix.len() as i32 || pos.1 >= matrix[0].len() as i32 {
            return 0;
        }

        let k = matrix[pos.0 as usize][pos.1 as usize];
        if k != n || visited.contains(&pos) { return 0; }
        visited.insert(pos);
        if k == 9 { return 1; }

        trail((pos.0 - 1, pos.1), n + 1, matrix, visited) +
        trail((pos.0 + 1, pos.1), n + 1, matrix, visited) +
        trail((pos.0, pos.1 - 1), n + 1, matrix, visited) +
        trail((pos.0, pos.1 + 1), n + 1, matrix, visited)
    }

    heads
        .into_iter()
        .fold(0, |acc, head| {
            acc + trail(head, 0, &matrix, &mut HashSet::new())
        })
}

pub fn part_2(input: String) -> i32 {
    let mut heads: Vec<Position> = vec![];
    let mut matrix: Vec<Vec<u8>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut matrix, (y, line)| {
            let line = line
                .split("")
                .filter_map(|str| str.parse::<u8>().ok())
                .enumerate()
                .map(|(x, n)| {
                    if n == 0 { heads.push((y as i32, x as i32)) }
                    n
                })
                .collect::<Vec<u8>>();

            matrix.push(line);
            matrix
        });

    fn trail(pos: Position, n: u8, matrix: &Vec<Vec<u8>>) -> i32 {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= matrix.len() as i32 || pos.1 >= matrix[0].len() as i32 {
            return 0;
        }

        let k = matrix[pos.0 as usize][pos.1 as usize];
        if k != n { return 0; }
        if k == 9 { return 1; }

        trail((pos.0 - 1, pos.1), n + 1, matrix) +
        trail((pos.0 + 1, pos.1), n + 1, matrix) +
        trail((pos.0, pos.1 - 1), n + 1, matrix) +
        trail((pos.0, pos.1 + 1), n + 1, matrix)
    }

    heads
        .into_iter()
        .fold(0, |acc, head| {
            acc + trail(head, 0, &matrix)
        })
}