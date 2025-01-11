use std::collections::{HashMap, HashSet};


pub fn part_1(input: String) -> i32 {
    let (dict, list) = parse(input);

    fn can_be_madeof(str: String, from: &HashSet<String>, cache: &mut HashMap<String, bool>) -> bool {
        match cache.get(&str) {
            None => {
                let n = str.len().min(8);
                let result = (1..=n)
                    .map(|i| str.split_at(i))
                    .filter(|(before, _)| from.contains(&before.to_string()))
                    .any(|(before, after)| after.is_empty() || can_be_madeof(after.to_string(), from, cache));

                cache.insert(str, result);
                result
            },
            Some(v) => *v,
        }
    }

    let mut cache = HashMap::new();
    list
        .into_iter()
        .filter(|each| can_be_madeof(each.clone(), &dict, &mut cache))
        .count() as i32
}

pub fn part_2(input: String) -> i128 {
    let (dict, list) = parse(input);
    let mut cache = HashMap::from([
        (String::from(""), 1)
    ]);

    fn combinations(str: String, from: &HashSet<String>, cache: &mut HashMap<String, i128>) -> i128 {
        match cache.get(&str) {
            None => {
                let n = str.len().min(8);
                let result = (1..=n)
                    .map(|i| str.split_at(i))
                    .filter(|(before, _)| from.contains(&before.to_string()))
                    .map(|(before, after)| combinations(after.to_string(), from, cache))
                    .sum();

                cache.insert(str, result);
                result
            },
            Some(v) => *v,
        }
    }

    list
        .into_iter()
        .map(|each| combinations(each.clone(), &dict, &mut cache))
        .sum()
}

fn parse(input: String) -> (HashSet<String>, Vec<String>) {
    let mut lines = input.lines();

    (
        lines
            .next()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect(),
        lines
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect()
    )
}