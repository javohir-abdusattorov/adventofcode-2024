use std::collections::HashMap;
use itertools::Itertools;


pub fn part_1(input: String) -> i128 {
    const STEPS: u32 = 2000;

    input
        .lines()
        .filter_map(|str| str.parse::<i128>().ok())
        .map(|number| run(number, STEPS))
        .sum()
}

pub fn part_2(input: String) -> i32 {
    const STEPS: usize = 2000;

    let numbers = input
        .lines()
        .filter_map(|str| str.parse::<i128>().ok())
        .collect();

    parallel(numbers, STEPS);

    0
}

fn parallel(numbers: Vec<i128>, steps: usize) {
    let n = numbers.len();
    let mut matrix = numbers
        .clone()
        .into_iter()
        .map(|number| {
            let mut row = vec![(0, 0, 0); steps];
            row[0] = (number, price_of(&number), 0);
            row
        })
        .collect::<Vec<Vec<(i128, i128, i128)>>>();

    let mut map = numbers
        .clone()
        .into_iter()
        .map(|number| HashMap::new())
        .collect::<Vec<HashMap<[i128; 4], i128>>>();

    for x in 1..steps {
        for y in 0..n {
            let (number, prev_price, change) = matrix[y][x - 1];
            let number = compute(number);
            let price = price_of(&number);

            matrix[y][x] = (
                number,
                price,
                price - prev_price,
            );

            if x > 2 {
                let sequence = [
                    matrix[y][x - 3].2,
                    matrix[y][x - 2].2,
                    matrix[y][x - 1].2,
                    matrix[y][x].2,
                ];

                if let Some(prev) = map[y].get(&sequence) {
                    if price > *prev {
                        map[y].insert(sequence.clone(), price);
                    }
                }
                else {
                    map[y].insert(sequence.clone(), price);
                }
            }
        }
    }

    for y in 0..n {
        let current_map = &map[y];
        let mut new: HashMap<[i128; 4], i128> = HashMap::new();
        for other_y in 0..n {
            if other_y == y { continue }

            for (other_key, other_price) in &map[other_y] {
                if let Some(current_price) = current_map.get(other_key) {
                    *new.entry(other_key.clone()).or_insert(*current_price) += *other_price;
                }
            }
        }

        let mut max = 0;
        let mut max_s = [0; 4];
        for (sequence, price) in new {
            if price > max {
                max = price;
                max_s = sequence;
            }
        }
        println!("[{y}] max: {max}; sequence: {max_s:?}");
    }
}

fn run(number: i128, steps: u32) -> i128 {
    (0..steps).fold(number, |number, _| compute(number))
}

fn compute(mut number: i128) -> i128 {
    let step = |number: i128, result: i128| { (number ^ result) % 16777216 };

    number = step(number, number * 64);
    number = step(number, (number as f64 / 32.0).floor() as i128);
    number = step(number, number * 2048);

    number
}

fn price_of(number: &i128) -> i128 {
    number.to_string().chars().last().unwrap().to_string().parse::<i128>().unwrap()
}