use std::{collections::{HashMap, HashSet}, fs::File};
use std::{io::Write};
use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    AND, OR, XOR
}

impl Operation {
    fn eval(&self, l: bool, r: bool) -> bool {
        match self {
            Operation::AND => l && r,
            Operation::OR => l || r,
            Operation::XOR => l != r,
        }
    }

    fn graph_id(&self, i: i32) -> String {
        match self {
            Operation::AND => format!("AND_{i:?}"),
            Operation::OR => format!("OR_{i:?}"),
            Operation::XOR => format!("XOR_{i:?}"),
        }
    }

    fn print_graph(&self, i: i32) -> String {
        match self {
            Operation::AND => format!("{} [shape=triangle, label=\"AND\", color=black]\n", self.graph_id(i)),
            Operation::OR => format!("{} [shape=triangle, label=\"OR\", color=black]\n", self.graph_id(i)),
            Operation::XOR => format!("{} [shape=triangle, label=\"XOR\", color=black]\n", self.graph_id(i)),
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "XOR" => Operation::XOR,
            _ => panic!("Invalid from value: {value} for Operation")
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    left: String,
    right: String,
    output: String,
    operation: Operation,
}

impl Gate {
    fn can_eval(&self, wires: &HashMap<String, bool>) -> bool {
        wires.contains_key(&self.left) && wires.contains_key(&self.right)
    }

    fn eval(&self, wires: &mut HashMap<String, bool>) -> Option<bool> {
        if !self.can_eval(wires) {
            None
        }
        else {
            let output = self.operation.eval(*wires.get(&self.left).unwrap(), *wires.get(&self.right).unwrap());
            // println!("{} {:?} {} -> {output}", self.left, self.operation, self.right);
            wires.insert(self.output.clone(), output);

            Some(output)
        }
    }
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        let (input, output) = value.split(" -> ").next_tuple().unwrap();
        let (l, operation, r) = input.split_whitespace().next_tuple().unwrap();

        Gate {
            left: l.to_string(),
            right: r.to_string(),
            output: output.to_string(),
            operation: Operation::from(operation),
        }
    }
}

pub fn part_1(input: String) -> i32 {
    let (mut wires, mut gates) = parse(input);
    run(&mut wires, &mut gates);

    let binary = collect_into_binary(wires, "z");
    println!("binary = {binary:?}");
    let int = binary_to_int(binary);
    println!("int = {int:?}");

    int as i32
}

pub fn part_2(input: String) -> i32 {
    let (mut wires, mut gates) = parse(input);

    let mut output = File::create("./outputs/24-2.txt").unwrap();
    let graph = print_graph(gates.clone());
    output.write_all(graph.as_bytes());

    let x_binary = binary_to_int(collect_into_binary(wires.clone(), "x"));
    println!("[x] binary = {x_binary:?}");
    let y_binary = binary_to_int(collect_into_binary(wires.clone(), "y"));
    println!("[y] binary = {y_binary:?}");
    run(&mut wires, &mut gates);

    let z_binary = collect_into_binary(wires.clone(), "z");
    println!("[z] binary = {z_binary}");
    let int = binary_to_int(z_binary.clone());
    println!("[z] int = {int:?}");
    let int_calculated = x_binary + y_binary;
    println!("[z] should be:");
    println!("[z] int = {int_calculated:?}");
    println!("[z] binary = {:b}", int_calculated);
    println!("[z] is-equal = {}", int == int_calculated);
    println!("");

    let swaps = [
        "z36",
        "nwq",
        "z18",
        "fvw",
        "mdb",
        "z22",
        "wpq",
        "grf",
    ];

    println!("{}", swaps.into_iter().sorted().join(","));
    int as i32
}

fn is_calculated(x_binary: String, y_binary: String, z_binary: String, offset: usize) -> bool {
    let x_binary = binary_to_int(x_binary[x_binary.len() - offset..].to_owned());
    let y_binary = binary_to_int(y_binary[y_binary.len() - offset..].to_owned());
    let z_binary = binary_to_int(z_binary[z_binary.len() - offset..].to_owned());
    println!("- [{offset}] x_binary = {x_binary}");
    println!("- [{offset}] y_binary = {y_binary}");
    println!("- [{offset}] z_binary = {z_binary}");
    println!("");

    x_binary + y_binary == z_binary
}

fn print_graph(gates: Vec<Gate>) -> String {
    let mut str = String::new();

    gates
        .iter()
        .map(|gate| [gate.left.clone(), gate.right.clone(), gate.output.clone()])
        .flatten()
        .collect::<HashSet<String>>()
        .into_iter()
        .map(|node| {
            let color = match node.chars().next().unwrap() {
                'x' => "blue",
                'y' => "green",
                'z' => "red",
                _ => "black",
            };

            format!("{node} [shape=box, color={color}]\n")
        })
        .for_each(|node| str.push_str(node.as_str()));

    let mut counter = HashMap::from([
        (Operation::AND, 0),
        (Operation::OR, 0),
        (Operation::XOR, 0),
    ]);

    gates
        .iter()
        .for_each(|gate| {
            let count = counter.get_mut(&gate.operation).unwrap();
            let operator_id = gate.operation.graph_id(*count);
            let operator = gate.operation.print_graph(*count);
            *count += 1;

            str.push_str(operator.as_str());
            str.push_str(format!("{} -- {operator_id}\n", gate.left).as_str());
            str.push_str(format!("{} -- {operator_id}\n", gate.right).as_str());
            str.push_str(format!("{operator_id} -- {}\n\n", gate.output).as_str());
        });

    str
}

fn run(wires: &mut HashMap<String, bool>, gates: &mut Vec<Gate>) {
    loop {
        let mut i = 0;
        while i < gates.len() {
            if let Some(output) = gates[i].eval(wires) {
                gates.remove(i);
            }
            else {
                i += 1;
            }
        }

        if gates.is_empty() {
            break;
        }
    }
}

fn collect_into_binary(wires: HashMap<String, bool>, char: &str) -> String {
    wires
        .into_iter()
        .filter(|(wire, val)| wire.starts_with(char))
        .sorted_by_key(|(wire, _)| wire.clone())
        .map(|(wire, val)| if val { '1' } else { '0' })
        .rev()
        .collect()
}

fn binary_to_int(binary: String) -> isize {
    isize::from_str_radix(&binary, 2).unwrap()
}

fn parse(input: String) -> (HashMap<String, bool>, Vec<Gate>) {
    let (wires, gates) = input.split("\n\n").next_tuple().unwrap();

    (
        wires
            .lines()
            .map(|line| line.split(": ").next_tuple::<(&str, &str)>().unwrap())
            .map(|(wire, value)| (wire.to_string(), value == "1"))
            .collect(),
        gates
            .lines()
            .map(|line| Gate::from(line))
            .collect()
    )
}