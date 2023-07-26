use std::io::{BufRead, BufReader};

const IMPORTANT_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub fn day_main(filename: &str, part: u8) {
    let mut register: i64 = 1;
    let mut cycles: Vec<i64> = Vec::with_capacity(256);
    let f = std::fs::File::open(filename).expect("Could not open file");
    for line_ in BufReader::new(f).lines() {
        if let Ok(line) = line_ {
            let mut parts = line.split_ascii_whitespace();
            match parts.next() {
                Some("addx") => {
                    cycles.push(register);
                    cycles.push(register);
                    let value: i64 = parts.next().unwrap().parse().unwrap();
                    register += value;
                }
                Some("noop") => {
                    cycles.push(register);
                }
                Some(s) => {
                    eprintln!("Unknown instruction {}", s);
                }
                _ => (),
            }
        }
    }
    match part {
        1 => part1_main(&cycles),
        2 => part2_main(&cycles),
        _ => panic!("unimplemented"),
    }
}

fn part2_main(cycles: &Vec<i64>) {
    for row in 0..6 {
        for column in 0..40 {
            let idx: usize = 40 * row + column;
            if (column as i64).abs_diff(cycles[idx]) <= 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn part1_main(cycles: &Vec<i64>) {
    let mut sum: i64 = 0;
    for &ic in IMPORTANT_CYCLES.iter() {
        let cycle_value = (ic as i64) * cycles[ic - 1];
        println!("{} * {} = {}", ic, cycles[ic - 1], cycle_value);
        sum += cycle_value;
    }
    println!("sum: {}", sum);
}
