use std::collections::{BinaryHeap, HashMap};
use std::io::Read;
use std::time::Instant;
use std::iter;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let (day, part) = read_day();
    let input = read_input(day, part);

    println!("Running for day #{day} part #{part}");

    let instant = Instant::now();
    let output = match (day, part) {
        (1, 1)  => day_1_part_1(input),
        (1, 2)  => day_1_part_2(input),
        (2, 1) => day_2_part_1(input),
        (2, 2) => day_2_part_2(input),
        (3, 1) => day_3_part_1(input),
        (3, 2) => day_3_part_2(input),
        (4, 1) => day_4_part_1(input),
        (4, 2) => day_4_part_2(input),
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

fn day_3_part_2(_: String) -> i32 {
    0
}


fn day_4_part_1(_: String) -> i32 {
    0
}

fn day_4_part_2(_: String) -> i32 {
    0
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