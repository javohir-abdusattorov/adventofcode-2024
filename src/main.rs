#![allow(warnings)]

use std::io::Read;
use std::time::Instant;
use itertools::Itertools;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    let (day, part) = read_day();
    let input = read_input(day, part);

    println!("Running for day #{day} part #{part}");

    let instant = Instant::now();
    let output = match (day, part) {
        (1, 1) => day_1::part_1(input),
        (1, 2) => day_1::part_2(input),

        (2, 1) => day_2::part_1(input),
        (2, 2) => day_2::part_2(input),

        (3, 1) => day_3::part_1(input),
        (3, 2) => day_3::part_2(input),

        (4, 1) => day_4::part_1(input),
        (4, 2) => day_4::part_2(input),

        (5, 1) => day_5::part_1(input),
        (5, 2) => day_5::part_2(input),

        (6, 1) => day_6::part_1(input),
        (6, 2) => day_6::part_2(input),

        (7, 1) => day_7::part_1(input) as i32,
        (7, 2) => day_7::part_2(input) as i32,

        (8, 1) => day_8::part_1(input),
        (8, 2) => day_8::part_2(input),

        (9, 1) => day_9::part_1(input) as i32,
        (9, 2) => day_9::part_2(input) as i32,
        _ => panic!("Solution not implemented for day #{day} part #{part}")
    };

    println!("Answer: {output}");
    println!("Time: {} ms", instant.elapsed().as_millis())
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