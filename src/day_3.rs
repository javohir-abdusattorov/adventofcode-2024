use regex::Regex;


pub fn part_1(input: String) -> i32 {
    Regex
        ::new(r"mul\([0-9]+,[0-9]+\)")
        .unwrap()
        .find_iter(&input)
        .fold(0, |acc, str| {
            acc + str
                .as_str()
                .to_string()
                .replace(")", "")
                .replace("mul(", "")
                .split(",")
                .filter_map(|str| str.parse::<i32>().ok())
                .fold(1, |r, n| r * n)
        })
}

pub fn part_2(input: String) -> i32 {
    let mut can = true;
    Regex
        ::new(r"(mul\([0-9]+,[0-9]+\))|(don\'t\(\))|(do\(\))")
        .unwrap()
        .find_iter(&input)
        .fold(0, |acc, str| {
            let str = str.as_str();
            match str {
                "do()" => { can = true; acc },
                "don't()" => { can = false; acc },
                _ if can => acc + str
                    .replace(")", "")
                    .replace("mul(", "")
                    .split(",")
                    .filter_map(|str| str.parse::<i32>().ok())
                    .fold(1, |r, n| r * n),
                _ => { acc },
            }
        })
}