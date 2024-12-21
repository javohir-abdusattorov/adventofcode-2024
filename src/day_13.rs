use std::collections::HashSet;
use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
    type Position = (i128, i128);

    let slots = input
        .split("\n\n")
        .map(|line| {
            line.replace("Button A: X+", "")
                .replace("Button B: X+", "")
                .replace("Prize: X=", "")
                .replace("Y+", "")
                .replace("Y+", "")
                .replace("Y=", "")
                .split("\n")
                .map(|each| {
                    each.split(", ")
                        .filter_map(|str| str.parse::<i128>().ok())
                        .next_tuple::<Position>()
                        .unwrap()
                })
                .next_tuple::<(Position, Position, Position)>()
                .unwrap()
        })
        .collect::<Vec<(Position, Position, Position)>>();

    let mut token = 0;
    for ((ax, ay), (bx, by), (px, py)) in slots {
        let a = (px * by - py * bx) / (ax * by - ay * bx);
        let b = (py * ax - px * ay) / (ax * by - ay * bx);

        let x = ax * a + bx * b;
        let y = ay * a + by * b;

        token += if (x, y) == (px, py) {
            a * 3 + b
        } else {
            0
        }
    }

    token as i32
}

pub fn part_2(input: String) -> i128 {
    type Position = (i128, i128);

    let slots = input
        .split("\n\n")
        .map(|line| {
            line.replace("Button A: X+", "")
                .replace("Button B: X+", "")
                .replace("Prize: X=", "")
                .replace("Y+", "")
                .replace("Y+", "")
                .replace("Y=", "")
                .split("\n")
                .map(|each| {
                    each.split(", ")
                        .filter_map(|str| str.parse::<i128>().ok())
                        .next_tuple::<Position>()
                        .unwrap()
                })
                .next_tuple::<(Position, Position, Position)>()
                .unwrap()
        })
        .collect::<Vec<(Position, Position, Position)>>();

    let mut token = 0;
    for ((ax, ay), (bx, by), (mut px, mut py)) in slots {
        px += 10000000000000;
        py += 10000000000000;

        let a = (px * by - py * bx) / (ax * by - ay * bx);
        let b = (py * ax - px * ay) / (ax * by - ay * bx);

        let x = ax * a + bx * b;
        let y = ay * a + by * b;

        token += if (x, y) == (px, py) {
            a * 3 + b
        } else {
            0
        }
    }

    token
}

pub fn part_1_try_dp(input: String) -> i32 {
    type Position = (u32, u32);

    let slots = input
        .split("\n\n")
        .map(|line| {
            line.replace("Button A: X+", "")
                .replace("Button B: X+", "")
                .replace("Prize: X=", "")
                .replace("Y+", "")
                .replace("Y+", "")
                .replace("Y=", "")
                .split("\n")
                .map(|each| {
                    each.split(", ")
                        .filter_map(|str| str.parse::<u32>().ok())
                        .next_tuple::<Position>()
                        .unwrap()
                })
                .next_tuple::<(Position, Position, Position)>()
                .unwrap()
        })
        .collect::<Vec<(Position, Position, Position)>>();

    let min = |a: u32, b: u32, c: u32| -> (HashSet<u32>, HashSet<u32>) {
        let mut a_count = 0;
        let a_minimized = (1..(c / a))
            .filter(|count| {
                a_count += a;
                (c - a_count) % b == 0
            })
            .collect::<HashSet<u32>>();

        let mut b_count = 0;
        let b_minimized = (1..(c / b))
            .filter(|count| {
                b_count += b;
                (c - b_count) % a == 0
            })
            .collect::<HashSet<u32>>();

        (
            a_minimized,
            b_minimized,
        )
    };

    let mut token = 0;
    for (a, b, c) in slots.iter() {
        let (x_a, x_b) = min(a.0, b.0, c.0);
        let (y_a, y_b) = min(a.1, b.1, c.1);

        let mut a_final = x_a.intersection(&y_a).map(|i| *i).collect::<Vec<u32>>();
        let mut b_final = x_b.intersection(&y_b).map(|i| *i).collect::<Vec<u32>>();

        let mut minimum = u32::MAX;
        for a_final in a_final {
            let remaining_x = c.0 - (a_final) * a.0;
            let remaining_y = c.1 - (a_final) * a.1;

            let b_price = b_final.iter().find(|b_final| {
                let b_remaining_x = (**b_final) * b.0;
                let b_remaining_y = (**b_final) * b.1;

                b_remaining_x == remaining_x && b_remaining_y == remaining_y
            });

            if let Some(b_price) = b_price {
                let a_price = a_final * 3;
                println!("a_price = {a_price:?}");
                println!("b_price = {b_price:?}");
                minimum = minimum.min(a_price + b_price);
            }
        }
        
        if minimum != u32::MAX {
            println!("minimum = {minimum:?}");
            println!("");
            token += minimum
        }
    }

    token as i32
}