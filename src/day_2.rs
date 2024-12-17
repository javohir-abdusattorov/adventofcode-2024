
pub fn part_1(input: String) -> i32 {
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

pub fn part_2(input: String) -> i32 {
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