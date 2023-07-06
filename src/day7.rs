#![allow(unused_variables)]
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

const PART_ONE_SIZE_LIMIT: u32 = 100000;

#[derive(Debug)]
struct FSObject {
    is_dir: bool,
    size: u32,
}
pub fn day_main(filename: &str) {
    let f = std::fs::File::open(filename).expect("Unable to read file");
    let lines = BufReader::new(f).lines();
    let mut paths: HashMap<String, FSObject> = HashMap::with_capacity(256);
    let mut cwd: Vec<String> = Vec::with_capacity(16);
    for line_ in lines {
        if let Ok(line) = line_ {
            interpret_line(&line, &mut paths, &mut cwd);
        }
    }
    let mut combined_size: u32 = 0;
    for (dirname, fsobj) in paths {
        if fsobj.is_dir && fsobj.size <= PART_ONE_SIZE_LIMIT {
            combined_size += fsobj.size;
        }
    }
    println!("Combined size: {}", combined_size);
}

fn interpret_line(line: &str, paths: &mut HashMap<String, FSObject>, cwd: &mut Vec<String>) {
    let mut components = line.split_ascii_whitespace();
    if let Some(first) = components.next() {
        let second = components.next().unwrap();
        match first {
            // command
            "$" => {
                match second {
                    "cd" => {
                        let newdir = components.next().unwrap();
                        match newdir {
                            "/" => {
                                cwd.clear();
                                cwd.push("".to_owned()); // leading space for generating full paths
                            }
                            ".." => {
                                cwd.pop();
                            }
                            _ => {
                                cwd.push(newdir.to_owned());
                            }
                        }
                    }
                    _ => {} // `ls` requires no action
                }
            }
            // ls output
            "dir" => {
                let mut this_path = cwd.as_slice().join("/");
                this_path.push('/');
                this_path.push_str(second);
                paths.insert(
                    this_path,
                    FSObject {
                        is_dir: true,
                        size: 0,
                    },
                );
            }
            _ => {
                if let Ok(size) = first.parse::<u32>() {
                    let mut this_path = cwd.as_slice().join("/");
                    this_path.push('/');
                    this_path.push_str(second);
                    paths.insert(
                        this_path,
                        FSObject {
                            is_dir: false,
                            size,
                        },
                    );
                    let mut to_update: HashSet<String> = HashSet::with_capacity(cwd.len());
                    for i in 0..=cwd.len() {
                        let mut dirname = cwd[..i].join("/");
                        if dirname.is_empty() {
                            dirname = "/".to_owned();
                        }
                        to_update.insert(dirname);
                    }
                    for dirname in to_update {
                        if let Some(d) = paths.get_mut(&dirname) {
                            d.size += size;
                        }
                    }
                } else {
                    eprintln!("Unable to understand line '{} {}'", first, second);
                }
            }
        }
    }
}
