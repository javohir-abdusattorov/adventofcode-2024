use std::collections::{HashMap, HashSet};
use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
    let relations = parse(input);
    let mut sets = HashSet::new();

    for (first, first_neighbours) in &relations {
        for second in first_neighbours {
            relations
                .get(second)
                .unwrap()
                .into_iter()
                .filter(|third| first_neighbours.contains(third))
                .map(|third| [first.clone(), second.clone(), third.clone()])
                .filter(|set| set.iter().any(|node| node.starts_with("t")))
                .for_each(|mut set| {
                    set.sort();
                    sets.insert(set);
                });
        }
    }

    sets.len() as i32
}

pub fn part_2(input: String) -> i32 {
    let relations = parse(input);
    let mut max_c = 0;
    let mut max_s = String::new();

    for (first, first_neighbours) in &relations {
        let mut map = HashMap::new();

        for second in first_neighbours {
            let mut third = relations
                .get(second)
                .unwrap()
                .into_iter()
                .filter(|third| first_neighbours.contains(third))
                .map(|third| third.clone())
                .chain([second.clone(), first.clone()])
                .collect::<Vec<String>>();

            if third.len() <= 3 { continue }

            third.sort();
            *map.entry(third).or_insert(1) += 1;
        }

        map
            .into_iter()
            .filter(|(set, count)| set.len() == *count)
            .for_each(|(set, count)| {
                if count > max_c {
                    max_c = count;
                    max_s = set.join(",");
                }
            });
    }

    println!("max_c = {max_c:?}");
    println!("max_s = {max_s:?}");

    0
}

fn parse(input: String) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|str| {
            str
                .split("-")
                .map(|str| str.to_string())
                .next_tuple::<(String, String)>()
                .unwrap()
        })
        .fold(HashMap::new(), |mut map: HashMap<String, Vec<String>>, (from, to)| {
            map.entry(from.clone()).or_insert(Vec::new()).push(to.clone());
            map.entry(to.clone()).or_insert(Vec::new()).push(from.clone());

            map
        })
}