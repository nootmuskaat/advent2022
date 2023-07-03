use std::fmt;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn from_lines(lines: &mut Vec<String>) -> Self {
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
        Self { stacks }
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

pub fn day_main(filename: &str) {
    let f = std::fs::File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(f).lines();
    let mut first_part = Vec::with_capacity(16);
    for line_ in lines {
        if let Ok(lin) = line_ {
            if lin.len() == 0 {
                break;
            } else {
                first_part.push(lin);
            }
        }
    }
    let crates = Crates::from_lines(&mut first_part);
    println!("{}", crates);
}
