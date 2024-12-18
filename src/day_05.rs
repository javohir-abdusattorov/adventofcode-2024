use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
    let mut parts = input.split("\n\n");
    let m = parts
        .next()
        .unwrap()
        .split("\n")
        .fold(HashMap::new(), |mut m, line| {
            let (a, b) = line.split("|").filter_map(|str| str.parse::<i32>().ok()).next_tuple().unwrap();
            m.entry(a).or_insert(Vec::new()).push(b);

            m
        });

    parts
        .next()
        .unwrap()
        .split("\n")
        .fold(0, |n, line| {
            let numbers = line.split(",").filter_map(|str| str.parse::<i32>().ok()).collect::<Vec<i32>>();
            for i in 1..numbers.len() {
                if let Some(ordering) = m.get(&numbers[i - 1]) {
                    if ordering.contains(&numbers[i]) {
                        continue;
                    }
                }

                return n;
            }

            n + numbers[numbers.len() / 2]
        })
}

pub fn part_2(input: String) -> i32 {
    let mut parts = input.split("\n\n");
    let m = parts
        .next()
        .unwrap()
        .split("\n")
        .fold(HashMap::new(), |mut m, line| {
            let (a, b) = line.split("|").filter_map(|str| str.parse::<i32>().ok()).next_tuple().unwrap();
            m.entry(a).or_insert(Vec::new()).push(b);

            m
        });

    parts
        .next()
        .unwrap()
        .split("\n")
        .map(|line| line.split(",").filter_map(|str| str.parse::<i32>().ok()).collect::<Vec<i32>>())
        .filter(|numbers| {
            !numbers
                .iter()
                .zip(numbers.iter().skip(1))
                .all(|(prev, current)| {
                    let ordering = m.get(&prev);
                    ordering.is_some() && ordering.unwrap().contains(current)
                })
        })
        .map(|mut numbers| {
            numbers.sort_by(|a, b| {
                let a_ordering = m.get(a);
                let b_ordering = m.get(b);

                if let Some(a) = a_ordering {
                    if a.contains(b) {
                        return Ordering::Greater;
                    }
                }

                if let Some(b) = b_ordering {
                    if b.contains(a) {
                        return Ordering::Less;
                    }
                }

                std::cmp::Ordering::Equal
            });

            numbers
        })
        .fold(0, |n, numbers| n + numbers[numbers.len() / 2])
}