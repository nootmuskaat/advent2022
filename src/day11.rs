use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_main(filename: &str, _part: u8) {
    let f = File::open(filename).expect("Unable to open file");
    let monkeys = create_monkeys(f);
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}: {:?}", i, monkey);
    }
}

const OPERATION_LINE: &str  = r"Operation: new = old (.) (\d+|old)";
const QUOTIENT_LINE: &str = r"Test: divisible by (\d+)";
const IF_TRUE_LINE: &str = r"If true: throw to monkey (\d+)";
const IF_FALSE_LINE: &str = r"If false: throw to monkey (\d+)";

fn create_monkeys(f: File) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut buffer: Vec<String> = Vec::with_capacity(5);

    for line_ in BufReader::new(f).lines() {
        if let Ok(line) = line_ {
            if line.len() == 0 {
                monkeys.push(create_monkey(&buffer));
                buffer.clear();
            } else if line.starts_with("Monkey") {
                continue;
            } else {
                buffer.push(line);
            }
        }
    }
    if buffer.len() == 5 {
        monkeys.push(create_monkey(&buffer));
    }
    monkeys
}

fn create_monkey(buffer: &Vec<String>) -> Monkey {
    let operation_line: Regex = Regex::new(&OPERATION_LINE).unwrap();
    let quotient_line: Regex = Regex::new(&QUOTIENT_LINE).unwrap();
    let if_true_line: Regex = Regex::new(&IF_TRUE_LINE).unwrap();
    let if_false_line: Regex = Regex::new(&IF_FALSE_LINE).unwrap();

    let items = read_initial_items(&buffer[0]);
    let operation_parts = operation_line.captures(&buffer[1]).unwrap();
    let operation = Operation::from_str(&operation_parts[1]);
    let operand: usize = match &operation_parts[2] {
        "old" => 0,
        s => s.parse::<usize>().unwrap(),
    };
    let quotient: usize = quotient_line.captures(&buffer[2]).unwrap()[1].parse().unwrap();
    let if_true: usize = if_true_line.captures(&buffer[3]).unwrap()[1].parse().unwrap();
    let if_false: usize = if_false_line.captures(&buffer[4]).unwrap()[1].parse().unwrap();
    Monkey {
        items,
        operation,
        operand,
        quotient,
        if_true,
        if_false,
    }
}

fn read_initial_items(s: &str) -> Vec<usize> {
    let parts: Vec<_> = s.split(": ").collect();
    let items: Vec<usize> = parts[1]
        .split(", ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    items
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation received"),
        }
    }

    fn perform(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    operand: usize,
    quotient: usize,
    if_true: usize,
    if_false: usize,
}

impl Monkey {}
