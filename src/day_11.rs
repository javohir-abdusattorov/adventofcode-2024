use std::collections::{HashMap, VecDeque};


pub fn part_1(input: String) -> i64 {
    let mut n = input
        .split_whitespace()
        .filter_map(|str| str.parse::<i64>().ok())
        .collect::<VecDeque<i64>>();

    for x in 0..75 {
        let mut i = 0;
        while i < n.len() {
            if n[i] == 0 {
                n[i] = 1;
            }
            else if ((n[i] as f64).log10().floor() as i32) % 2 != 0 {
                let mut first = n[i].to_string();
                let second = first.split_off(first.len() / 2).parse::<i64>().unwrap();

                n[i] = first.parse::<i64>().unwrap();
                i += 1;
                n.insert(i, second);
            }
            else {
                n[i] *= 2024;
            }
            i += 1;
        }
    }

    n.len() as i64
    
}

pub fn part_2(input: String) -> i64 {
    const CYLCE: i32 = 15;
    const PRECOMPUTE_CYLCLE: i32 = 5;

    fn precompute(n: i64, mini: &mut HashMap<i64, Vec<i64>>) {
        if mini.contains_key(&n) { return; }

        let stones = (0..PRECOMPUTE_CYLCLE).fold(vec![n], |mut stones, _| {
            blink(&mut stones);
            stones
        });

        mini.insert(n, stones.clone());

        stones.into_iter().for_each(|n| precompute(n, mini));
    }

    fn blink(stones: &mut Vec<i64>) {
        let mut i = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
            }
            else if ((stones[i] as f64).log10().floor() as i32) % 2 != 0 {
                let mut first = stones[i].to_string();
                let second = first.split_off(first.len() / 2).parse::<i64>().unwrap();

                stones[i] = first.parse::<i64>().unwrap();
                i += 1;
                stones.insert(i, second);
            }
            else {
                stones[i] *= 2024;
            }
            i += 1;
        }
    }

    let initial = input
        .split_whitespace()
        .filter_map(|str| str.parse::<i64>().ok())
        .fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });

    let precomputed: HashMap<i64, Vec<i64>> = initial
        .iter()
        .fold(HashMap::new(), |mut acc, (n, _)| {
            precompute(*n, &mut acc);
            acc
        });

    (0..CYLCE)
        .fold(initial, |major, _| {
            major.iter().fold(HashMap::new(), |mut major: HashMap<i64, i64>, (stone, n)| {
                precomputed
                    .get(stone)
                    .unwrap()
                    .iter()
                    .for_each(|child: &i64| *major.entry(*child).or_insert(0) += n);

                major
            })
        })
        .iter()
        .fold(0, |acc, (_, n)| acc + n)
}