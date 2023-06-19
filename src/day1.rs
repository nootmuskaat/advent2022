use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::env;
use std::fs;

fn filename() -> String {
    let args: Vec<String> = env::args().collect();
    let f = args[1].clone();
    println!("Will parse file {}", f);
    f
}

#[allow(dead_code)]
fn most_calories(filepath: &str) -> Result<usize, std::io::Error> {
    let mut count: usize = 0;
    let mut max_found: usize = 0;
    fs::read_to_string(&filepath)?
        .split("\n")
        .map(|line| line.parse::<usize>())
        .for_each(|line| match line {
            Ok(i) => {
                count += i;
                max_found = max_found.max(count);
            }
            Err(_) => {
                count = 0;
            }
        });
    Ok(max_found)
}

fn top_3_calories(filepath: &str) -> Result<usize, std::io::Error> {
    let mut most_calories: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(4);
    let mut count: usize = 0;
    fs::read_to_string(&filepath)?
        .split("\n")
        .chain(std::iter::once("")) // ensure we can push the last grouping
        .map(|line| line.parse::<usize>())
        .for_each(|line| match line {
            Ok(i) => {
                count += i;
            }
            Err(_) => {
                most_calories.push(Reverse(count));
                if most_calories.len() > 3 {
                    _ = most_calories.pop();
                }
                count = 0;
            }
        });
    Ok(most_calories.iter().fold(0, |total, i| total + i.0))
}

pub fn main() {
    let f = filename();
    // if let Ok(most_calories) = most_calories(&f) {
    if let Ok(most_calories) = top_3_calories(&f) {
        println!("The most calories available: {:?}", most_calories);
    } else {
        eprintln!("Failed to read file {}", f);
    }
}
