use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_main(filename: &str, _part: u8) {
    let f = File::open(filename).expect("Unable to open file");
    let mut monkeys = create_monkeys(f);
    let mut counts: Vec<usize> = vec![0; monkeys.len()];
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}: {:?}", i, monkey);
    }
    for round in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let (to_idx, val) = monkeys[i].inspect_item();
                counts[i] += 1;
                monkeys[to_idx].items.push_back(val);
            }
        }
    }
    counts.sort();
    println!("{:?}", counts);
    let most = counts.pop().unwrap();
    let next_most = counts.pop().unwrap();
    println!("{}", most * next_most);
}

const OPERATION_LINE: &str = r"Operation: new = old (.) (\d+|old)";
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

    let parts: Vec<_> = buffer[0].split(": ").collect();
    let items: VecDeque<usize> = parts[1]
        .split(", ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let operation_parts = operation_line.captures(&buffer[1]).unwrap();
    let operation = Operation::from_str(&operation_parts[1]);
    let operand: usize = match &operation_parts[2] {
        "old" => 0,
        s => s.parse::<usize>().unwrap(),
    };
    let quotient: usize = quotient_line.captures(&buffer[2]).unwrap()[1]
        .parse()
        .unwrap();
    let if_true: usize = if_true_line.captures(&buffer[3]).unwrap()[1]
        .parse()
        .unwrap();
    let if_false: usize = if_false_line.captures(&buffer[4]).unwrap()[1]
        .parse()
        .unwrap();
    Monkey {
        items,
        operation,
        operand,
        quotient,
        if_true,
        if_false,
    }
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
    items: VecDeque<usize>,
    operation: Operation,
    operand: usize,
    quotient: usize,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn inspect_item(&mut self) -> (usize, usize) {
        let mut new_val: usize;
        if let Some(item) = self.items.pop_front() {
            if self.operand == 0 {
                new_val = self.operation.perform(item, item);
            } else {
                new_val = self.operation.perform(item, self.operand);
            }
            new_val /= 3;
            if new_val % self.quotient == 0 {
                (self.if_true, new_val)
            } else {
                (self.if_false, new_val)
            }
        } else {
            panic!("This is bad");
        }
    }
}
