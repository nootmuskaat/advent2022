#[path = "common.rs"]
mod common;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_value(c: char) -> i32 {
    match c {
        'a'..='z' => (c as i32) - ('a' as i32) + 1,
        'A'..='Z' => (c as i32) - ('A' as i32) + 27,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_values() {
        assert_eq!(char_value('a'), 1);
        assert_eq!(char_value('z'), 26);
        assert_eq!(char_value('A'), 27);
        assert_eq!(char_value('Z'), 52);
    }
}

pub fn main() {
    let f = File::open(common::filename()).expect("Couldn't open file");
    let reader = BufReader::new(f);
    let mut priority_sum = 0;
    let mut middle: usize;
    let mut right_contents: HashSet<char>;
    let mut left_contents: HashSet<char>;

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            middle = line.len() / 2;
            right_contents = line[..middle].chars().collect();
            left_contents = line[middle..].chars().collect();
            for c in right_contents.intersection(&left_contents) {
                priority_sum += char_value(*c);
            }
            right_contents.clear();
            left_contents.clear();
        }
    }
    println!("Cumulative priorities: {}", priority_sum);
}
