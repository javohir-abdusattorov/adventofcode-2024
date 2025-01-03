use std::cell::Cell;
use itertools::Itertools;


pub fn part_1(input: String) -> i32 {
    let (reg_a, reg_b, reg_c, program) = parse(input);
    let mut out = compute(reg_a, reg_b, reg_c, program);

    println!("{}", out.iter().join(","));
    0
}

pub fn part_2( input: String ) -> u128 {
    let (reg_a, reg_b, reg_c, program) = parse(input);
    let mut queue = vec![(program.len() - 1, 0)];

    while let Some((offset, a)) = queue.pop() {
        for i in 0..8 {
            let reg_a = a * 8 + i;
            if compute(reg_a, reg_b, reg_c, program.clone())[..] == program[offset..] {
                if offset == 0 {
                    println!("A={reg_a}");
                    return reg_a;
                }

                queue.insert(0, (offset - 1, reg_a));
            }
        }
    }

    0
}

fn compute(reg_a: u128, reg_b: u128, reg_c: u128, program: Vec<u128>) -> Vec<u128> {
    let reg_a = Cell::new(reg_a);
    let reg_b = Cell::new(reg_b);
    let reg_c = Cell::new(reg_c);
    let exp = 2_u128;

    let mut ptr = 0;
    let mut out: Vec<u128> = Vec::new();

    let combo_operand = |n: u128| -> u128 {
        match n {
            4 => reg_a.get(),
            5 => reg_b.get(),
            6 => reg_c.get(),
            _ => n
        }
    };

    while ptr < program.len() {
        let instruction = program[ptr];
        let literal_operand = program[ptr + 1];
        let combo_operand = combo_operand(literal_operand);

        ptr += 2;
        match instruction {
            0 => {
                reg_a.set(reg_a.get() / exp.pow(combo_operand as u32));
            },
            1 => {
                reg_b.set(reg_b.get() ^ literal_operand);
            },
            2 => {
                reg_b.set(combo_operand % 8);
            },
            3 if reg_a.get() != 0 => {
                ptr = literal_operand as usize;
            },
            4 => {
                reg_b.set(reg_b.get() ^ reg_c.get());
            },
            5 => {
                out.push(combo_operand % 8);
            },
            6 => {
                reg_b.set(reg_a.get() / exp.pow(combo_operand as u32));
            },
            7 => {
                reg_c.set(reg_a.get() / exp.pow(combo_operand as u32));
            },
            _ => {},
        }
    }

    out
}

fn parse(input: String) -> (u128, u128, u128, Vec<u128>) {
    let removed = input
        .replace("Register A: ", "")
        .replace("Register B: ", "")
        .replace("Register C: ", "")
        .replace("Program: ", "");

    let (a, b, c, program) = removed
        .lines()
        .filter(|line| !line.is_empty())
        .next_tuple()
        .unwrap();

    (
        a.parse::<u128>().unwrap(),
        b.parse::<u128>().unwrap(),
        c.parse::<u128>().unwrap(),
        program
            .split(",")
            .filter_map(|str| str.parse::<u128>().ok())
            .collect()
    )
}