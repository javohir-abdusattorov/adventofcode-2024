#![allow(warnings)]

use std::io::Read;
use std::time::Instant;
use itertools::Itertools;

pub mod util {
    pub mod point;
}

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

fn main() {
    let (day, part) = read_day();
    let input = read_input(day, part);

    println!("Running for day #{day} part #{part}");

    let instant = Instant::now();
    let output = match (day, part) {
        (1, 1) => day_01::part_1(input),
        (1, 2) => day_01::part_2(input),

        (2, 1) => day_02::part_1(input),
        (2, 2) => day_02::part_2(input),

        (3, 1) => day_03::part_1(input),
        (3, 2) => day_03::part_2(input),

        (4, 1) => day_04::part_1(input),
        (4, 2) => day_04::part_2(input),

        (5, 1) => day_05::part_1(input),
        (5, 2) => day_05::part_2(input),

        (6, 1) => day_06::part_1(input),
        (6, 2) => day_06::part_2(input),

        (7, 1) => day_07::part_1(input) as i32,
        (7, 2) => day_07::part_2(input) as i32,

        (8, 1) => day_08::part_1(input),
        (8, 2) => day_08::part_2(input),

        (9, 1) => day_09::part_1(input) as i32,
        (9, 2) => day_09::part_2(input) as i32,

        (10, 1) => day_10::part_1(input),
        (10, 2) => day_10::part_2(input),

        (11, 1) => day_11::part_1(input) as i32,
        (11, 2) => day_11::part_2(input) as i32,

        (12, 1) => day_12::part_1(input),
        (12, 2) => day_12::part_2(input),

        (13, 1) => day_13::part_1(input),
        (13, 2) => day_13::part_2(input) as i32,

        (14, 1) => day_14::part_1(input),
        (14, 2) => day_14::part_2(input),

        (15, 1) => day_15::part_1(input),
        (15, 2) => day_15::part_2(input),

        (16, 1) => day_16::part_1(input),
        (16, 2) => day_16::part_2(input),

        (17, 1) => day_17::part_1(input),
        (17, 2) => day_17::part_2(input) as i32,

        (18, 1) => day_18::part_1(input),
        (18, 2) => day_18::part_2(input),

        (19, 1) => day_19::part_1(input),
        (19, 2) => day_19::part_2(input) as i32,

        (20, 1) => day_20::part_1(input),
        (20, 2) => day_20::part_2(input),

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