use std::{fmt::Debug, path::Display};

use crate::util::point::Point;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mechanisms {
    Lock, Key
}

#[derive(Clone, Copy, Debug)]
struct Schematic {
    mechanism: Mechanisms, 
    heights: [u8; 5],
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mechanism = if value.lines().next().unwrap().starts_with("#") {
            Mechanisms::Lock
        } else {
            Mechanisms::Key
        };

        let mut heights = [0; 5];
        for line in value.lines().skip(1).take(5) {
            line
                .split("")
                .filter(|cell| !cell.is_empty())
                .enumerate()
                .map(|(x, cell)| (x, cell == "#"))
                .filter(|(x, block)| *block)
                .for_each(|(x, _)| heights[x] += 1);
        }

        Schematic {
            mechanism: mechanism,
            heights: heights
        }
    }
}

impl PartialEq for Schematic {
    fn eq(&self, other: &Self) -> bool {
        self.heights
            .iter()
            .enumerate()
            .all(|(i, h)| {
                h + other.heights[i] <= 5
            })
    }
}

pub fn part_1(input: String) -> i32 {
    let (lock, keys) = parse(input);

    lock
        .into_iter()
        .map(|lock| {
            keys
                .iter()
                .filter(|key| lock == **key)
                .count() as i32
        })
        .sum()
}

pub fn part_2(input: String) -> i32 {
    0
}

fn parse(input: String) -> (Vec<Schematic>, Vec<Schematic>) {
    input
        .split("\n\n")
        .map(|matrix| Schematic::from(matrix))
        .partition(|schematic| schematic.mechanism == Mechanisms::Lock)
}