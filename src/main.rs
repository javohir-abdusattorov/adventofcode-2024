use std::collections::BinaryHeap;
use std::io::Read;
use std::time::Instant;
use std::iter;
use itertools::Itertools;

fn main() {
    let day = read_day();
    let input = read_input(day);

    println!("Running for day #{day}");

    let instant = Instant::now();
    let output = match day {
        1 => day_1(input),
        2 => day_2(input),
        3 => day_3(input),
        _ => panic!("Solution not implemented for day #{day}")
    };

    println!("Answer: {output}");
    println!("Time: {} ms", instant.elapsed().as_millis())
}

fn day_1(input: String) -> i32 {
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

fn day_2(input: String) -> i32 {
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

fn day_3(_: String) -> i32 {
    0
}

fn day_4(_: String) -> i32 {
    0
}

fn day_5(_: String) -> i32 {
    0
}


fn read_day() -> i32 {
    std::env::args()
        .skip(1)
        .take(1)
        .collect::<String>().parse::<i32>()
        .expect("Please input day number")
}

fn read_input(day: i32) -> String {
    let mut input: String = String::new();
    let input_path = format!("inputs/{}.txt", day);

    std::fs::File::open(input_path.clone())
        .expect(&format!("Could't open {} file", input_path))
        .read_to_string(&mut input)
        .expect(&format!("Could't read file: {}", input_path));

    input
}