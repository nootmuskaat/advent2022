#[path="common.rs"] mod common;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::io::{BufRead, BufReader, Lines};

struct InventoryFile {
    lines: Lines<BufReader<fs::File>>,
    completed: bool,
}

impl InventoryFile {
    fn new(file: fs::File) -> InventoryFile {
        InventoryFile {
            lines: BufReader::new(file).lines(),
            completed: false,
        }
    }
}

impl Iterator for InventoryFile {
    type Item = Option<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(res) = self.lines.next() {
            if let Ok(line) = res {
                if let Ok(i) = line.parse::<usize>() {
                    return Some(Some(i));
                } else {
                    return Some(None);
                }
            }
        }

        if !self.completed {
            self.completed = true;
            Some(None)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn top_calories(inv: &mut InventoryFile) -> usize {
    let mut count: usize = 0;
    let mut max_found: usize = 0;
    inv.into_iter().for_each(|line| match line {
        Some(i) => {
            count += i;
            max_found = max_found.max(count);
        }
        None => {
            count = 0;
        }
    });
    max_found
}

fn top_3_calories(inv: &mut InventoryFile) -> usize {
    let mut most_calories: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(4);
    let mut count: usize = 0;
    inv.into_iter().for_each(|line| match line {
        Some(i) => {
            count += i;
        }
        None => {
            most_calories.push(Reverse(count));
            if most_calories.len() > 3 {
                _ = most_calories.pop();
            }
            count = 0;
        }
    });
    most_calories.iter().fold(0, |total, i| total + i.0)
}

pub fn main() {
    let f = fs::File::open(common::filename()).expect("couldn't open file");
    let mut inv = InventoryFile::new(f);
    // let most_calories = top_calories(&mut inv);
    let most_calories = top_3_calories(&mut inv);
    println!("The most calories available: {:?}", most_calories);
}
