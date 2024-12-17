

pub fn part_1(input: String) -> i32 {
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

pub fn part_2(input: String) -> i32 {
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