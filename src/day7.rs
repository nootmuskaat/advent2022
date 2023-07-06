#![allow(unused_variables)]
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
enum FSObject {
    Directory {
        name: String,
        contents: Vec<FSObject>,
    },
    File {
        name: String,
        size: u32,
    },
}

pub fn day_main(filename: &str) {
    let f = std::fs::File::open(filename).expect("Unable to read file");
    let lines = BufReader::new(f).lines();
    for line_ in lines {
        if let Ok(line) = line_ {
            interpret_line(&line);
        }
    }
}

fn interpret_line(line: &str) {
    let mut components = line.split_ascii_whitespace();
    if let Some(first) = components.next() {
        let second = components.next().unwrap();
        match first {
            // command
            "$" => {
                match second {
                    "cd" => {
                        let newdir = components.next().unwrap();
                        // set pointer to the newdir
                    }
                    _ => {} // `ls` requires no action
                }
            }
            // dir contents
            "dir" => {
                todo!(); // second == dirname
            }
            _ => {
                if let Ok(size) = first.parse::<u32>() {
                    todo!(); // second == filename
                } else {
                    eprintln!("Unable to understand line '{} {}'", first, second);
                }
            }
        }
    }
}
