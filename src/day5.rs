use regex::Regex;
use std::fmt;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
enum Crane {
    CrateMover9000,
    CrateMover9001,
}

struct Crates {
    stacks: Vec<Vec<char>>,
    crane: Crane,
}

impl Crates {
    fn from_lines(lines: &mut Vec<String>, crane: Crane) -> Self {
        let num_columns: usize = lines
            .pop()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .len();
        let max_height = lines.len() * num_columns;
        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_columns);
        for _ in 0..num_columns {
            let v: Vec<char> = Vec::with_capacity(max_height);
            stacks.push(v);
        }
        while let Some(line) = lines.pop() {
            let row = line.chars().enumerate();
            for (pos, char) in row {
                match char {
                    'A'..='Z' => {
                        // 1 -> 0, 5 -> 1, 9 -> 2
                        let idx = (pos - 1) / 4;
                        stacks[idx].push(char);
                    }
                    _ => {}
                }
            }
        }
        Self { stacks, crane }
    }

    fn move_items(&mut self, amount: usize, from: usize, onto: usize) {
        match self.crane {
            Crane::CrateMover9000 => {
                for _ in 0..amount {
                    let c = self.stacks[from - 1].pop().expect(&format!(
                        "Invalid! move {} from {} to {}",
                        amount, from, onto
                    ));
                    self.stacks[onto - 1].push(c);
                }
            }
            Crane::CrateMover9001 => {
                let new_size = self.stacks[from - 1].len() - amount;
                let moved = self.stacks[from - 1].split_off(new_size);
                self.stacks[onto - 1].extend(moved);
            }
        }
    }

    fn stack_tops(&self) -> Vec<char> {
        let mut tops: Vec<char> = Vec::with_capacity(self.stacks.len());
        for stack in &self.stacks {
            tops.push(stack[stack.len() - 1]);
        }
        tops
    }
}

impl fmt::Display for Crates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut height: usize = 0;
        for stack in &self.stacks {
            height = height.max(stack.len());
        }
        while height > 0 {
            for stack in &self.stacks {
                if stack.len() >= height {
                    write!(f, "[{}] ", stack[height - 1])?;
                } else {
                    write!(f, "    ")?;
                }
            }
            write!(f, "\n")?;
            height -= 1;
        }
        for idx in 0..self.stacks.len() {
            write!(f, " {}  ", idx + 1)?;
        }
        Ok(())
    }
}

pub fn day_main(filename: &str, part: u8) {
    let f = std::fs::File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(f).lines();
    let mut first_part = Vec::with_capacity(16);
    loop {
        if let Some(Ok(lin)) = lines.next() {
            if lin.len() == 0 {
                break;
            } else {
                first_part.push(lin);
            }
        }
    }
    let crane = match part {
        1 => Crane::CrateMover9000,
        2 => Crane::CrateMover9001,
        _ => panic!("Unimplemented part!"),
    };
    let mut crates = Crates::from_lines(&mut first_part, crane);
    println!("Begining crate setup:\n{}", crates);
    // move 1 from 2 to 1
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    loop {
        if let Some(Ok(text)) = lines.next() {
            for cap in re.captures_iter(&text) {
                let amount = &cap[1].parse::<usize>().unwrap();
                let from = &cap[2].parse::<usize>().unwrap();
                let onto = &cap[3].parse::<usize>().unwrap();
                crates.move_items(*amount, *from, *onto);
            }
        } else {
            break;
        }
    }
    println!("After crate moves:\n{}", crates);
    println!("Tops: {:?}", crates.stack_tops().iter().collect::<String>());
}
