use std::{fs::File, io::Write};
use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
    type Position = (i32, i32);

    const XL: i32 = 101;
    const YL: i32 = 103;
    const N: i32 = 100;

    const XM: i32 = XL / 2;
    const YM: i32 = YL / 2;

    const SIDES: [((i32, i32), (i32, i32)); 4] = [
        ((0, XM), (0, YM)),
        ((XM, XL), (0, YM)),
        ((0, XM), (YM, YL)),
        ((XM, XL), (YM, YL)),
    ];

    let mut robots: Vec<(Position, Position)> = input
        .split("\n")
        .map(|line| {
            line.replace("p=", "")
                .replace("v=", "")
                .split(" ")
                .map(|each| {
                    each.split(",")
                        .filter_map(|str| str.parse::<i32>().ok())
                        .next_tuple::<Position>()
                        .unwrap()
                })
                .next_tuple::<(Position, Position)>()
                .unwrap()
        })
        .fold(Vec::new(), |mut acc, (p, v)| {
            acc.push((
                p,
                (v.0 * N, v.1 * N)
            ));
            acc
        });

    let calc = |p: i32, v: i32, l: i32| -> i32 {
        if v.is_positive() {
            let p1 = l - p;
            if v <= p1 {
                p + v
            } else {
                let v1 = v - p1;
                v1 % l
            }
        } else {
            let v1 = v + p;
            if v1 >= 0 {
                v1
            } else {
                let v2 = v1 % l;
                if v2 == 0 {
                    0
                } else {
                    l + v2
                }
            }
        }
    };

    let mut count = [0, 0, 0, 0];
    for ((px, py), (vx, vy)) in robots {
        let x = calc(px, vx, XL);
        let y = calc(py, vy, YL);

        if x == XM || y == YM {
            continue;
        }

        let index = SIDES.iter().find_position(|(sx, sy)| {
            (x >= sx.0) && (x <= sx.1) && (y >= sy.0) && (y <= sy.1)
        }).unwrap().0;
        count[index] += 1;
    }

    count.iter().fold(1, |acc, c| acc * c)
}

pub fn part_2(input: String) -> i32 {
    type Position = (i32, i32);

    const XL: i32 = 101;
    const YL: i32 = 103;

    const XM: i32 = XL / 2;
    const YM: i32 = YL / 2;

    let mut robots: Vec<(Position, Position)> = input
        .split("\n")
        .map(|line| {
            line.replace("p=", "")
                .replace("v=", "")
                .split(" ")
                .map(|each| {
                    each.split(",")
                        .filter_map(|str| str.parse::<i32>().ok())
                        .next_tuple::<Position>()
                        .unwrap()
                })
                .next_tuple::<(Position, Position)>()
                .unwrap()
        })
        .fold(Vec::new(), |mut acc, (p, v)| {
            acc.push((
                p,
                (v.0, v.1)
            ));
            acc
        });

    let calc = |p: i32, v: i32, l: i32| -> i32 {
        if v.is_positive() {
            let p1 = l - p;
            if v <= p1 {
                p + v
            } else {
                let v1 = v - p1;
                v1 % l
            }
        } else {
            let v1 = v + p;
            if v1 >= 0 {
                v1
            } else {
                let v2 = v1 % l;
                if v2 == 0 {
                    0
                } else {
                    l + v2
                }
            }
        }
    };

    let mut count = [0, 0, 0, 0];
    let mut output = File::create("./outputs/14-2.txt").unwrap();
    output.write_all(b"");

    let mut i = 1;
    loop {
        for j in 0..robots.len() {
            let ((px, py), (vx, vy)) = robots[j];
            robots[j].0.0 = calc(px, vx, XL);
            robots[j].0.1 = calc(py, vy, YL);
        }

        let mut s = String::new();
        let mut write = false;

        for y in 0..YL {
            let mut x = vec![" "; XL as usize + 1];
            let robots = robots
                .iter()
                .filter(|(position, _)| position.1 == y)
                .for_each(|(position, _)| {
                    x[position.0 as usize] = "X";
                });

            let x_line = format!("{}\n", x.join(""));
            s.push_str(x_line.as_str());

            if x_line.contains("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX") {
                write = true;
            }
        }

        if write {
            output.write(format!("{i} - iteration\n{s}\n\n").as_bytes());
            break;
        }

        i += 1;
    }

    i
}