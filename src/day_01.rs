use std::collections::{BinaryHeap, HashMap};
use std::iter;
use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
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

pub fn part_2(input: String) -> i32 {
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