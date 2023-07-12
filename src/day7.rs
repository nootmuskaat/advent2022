#![allow(unused_variables)]
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

const PART_ONE_SIZE_LIMIT: u32 = 100000;
const TOTAL_FS_SIZE: u32 = 70000000;
const NEEDED_FREE_SPACE: u32 = 30000000;

#[derive(Debug)]
struct FSObject {
    is_dir: bool,
    size: u32,
}
pub fn day_main(filename: &str, part: u8) {
    let f = std::fs::File::open(filename).expect("Unable to read file");
    let lines = BufReader::new(f).lines();
    let mut paths: HashMap<String, FSObject> = HashMap::with_capacity(256);
    paths.insert(
        "/".to_owned(),
        FSObject {
            is_dir: true,
            size: 0,
        },
    );
    let mut cwd: Vec<String> = Vec::with_capacity(16);
    for line_ in lines {
        if let Ok(line) = line_ {
            interpret_line(&line, &mut paths, &mut cwd);
        }
    }

    if part == 1 {
        let mut combined_size: u32 = 0;
        for (dirname, fsobj) in &paths {
            if fsobj.is_dir && fsobj.size <= PART_ONE_SIZE_LIMIT {
                combined_size += fsobj.size;
            }
        }
        println!("Combined size: {}", combined_size);
    } else {
        let root_size = paths.get("/").expect("We're missing root?").size;
        let need_to_free = NEEDED_FREE_SPACE - (TOTAL_FS_SIZE - root_size);
        let mut big_enough: Vec<(&String, &FSObject)> = paths
            .iter()
            .filter(|(&ref k, &ref v)| v.is_dir && v.size >= need_to_free)
            .collect();
        big_enough.sort_by(|a, b| a.1.size.cmp(&b.1.size));
        println!(
            "Smallest dir that's big enough: {}, size: {}",
            big_enough[0].0, big_enough[0].1.size
        );
    }
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
