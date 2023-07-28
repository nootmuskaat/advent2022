use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Item = usize;

pub fn day_main(filename: &str, part: u8) {
    let f = File::open(filename).expect("Unable to open file");
    let mut monkeys = create_monkeys(f);
    let mut counts: Vec<usize> = vec![0; monkeys.len()];
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}: {:?}", i, monkey);
    }
    let (rounds, destresser) = match part {
        1 => (20, StressManager::Divide(3)),
        2 => (10000, StressManager::Remainder(monkey_divisor(&monkeys))),
        _ => panic!("Not implemented"),
    };
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let (to_idx, val) = monkeys[i].inspect_item(&destresser);
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

fn monkey_divisor(monkeys: &Vec<Monkey>) -> usize {
    monkeys.iter().map(|m| m.quotient).fold(1, |x, y| x * y)
}

enum StressManager {
    Divide(Item),
    Remainder(Item),
}

impl StressManager {
    fn perform(&self, other: Item) -> Item {
        match self {
            StressManager::Divide(d) => other / d,
            StressManager::Remainder(d) => other % d,
        }
    }
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
    let items: VecDeque<Item> = parts[1]
        .split(", ")
        .map(|x| x.parse::<Item>().unwrap())
        .collect();
    let operation_parts = operation_line.captures(&buffer[1]).unwrap();
    let operation = Operation::from_str(&operation_parts[1]);
    let operand: Item = match &operation_parts[2] {
        "old" => 0,
        s => s.parse::<Item>().unwrap(),
    };
    let quotient: Item = quotient_line.captures(&buffer[2]).unwrap()[1]
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

    fn perform(&self, a: &Item, b: &Item) -> Item {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    operand: Item,
    quotient: Item,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn inspect_item(&mut self, destress: &StressManager) -> (usize, Item) {
        let mut item = self.items.pop_front().expect("Why is there no item?");
        if self.operand == 0 {
            item = self.operation.perform(&item, &item);
        } else {
            item = self.operation.perform(&item, &self.operand);
        }

        item = destress.perform(item);
        // let new_val = item.clone();
        if item % &self.quotient == 0 {
            (self.if_true, item)
        } else {
            (self.if_false, item)
        }
    }
}
