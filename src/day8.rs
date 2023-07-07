use std::io::{BufRead, BufReader};
use std::collections::HashSet;

type Row = Vec<u8>;
const TALLEST_POSSIBLE: u8 = 9;

pub fn day_main(filename: &str) {
    let f = std::fs::File::open(filename).expect("Unable to open file");
    let mut lines = BufReader::new(f).lines();
    let matrix = create_matrix(lines);
    // println!("{:?}", &matrix);
    let mut visible: HashSet<(usize, usize)> = HashSet::with_capacity(128);
    let mut tallest: Option<&u8> = None;
    for row in 0..matrix.len() {
        tallest = None;
        for col in 0..matrix[row].len() {
            match tallest {
                None => {
                    visible.insert((row, col));
                    tallest = Some(&matrix[row][col]);
                }
                Some(&tall) => {
                    if matrix[row][col] > tall {
                        visible.insert((row, col));
                        tallest = Some(&matrix[row][col]);
                    }
                }
            }
            if *tallest.unwrap() == TALLEST_POSSIBLE {
                break;
            }
        }
        tallest = None;
        for col in (0..matrix[row].len()).rev() {
            match tallest {
                None => {
                    visible.insert((row, col));
                    tallest = Some(&matrix[row][col]);
                }
                Some(&tall) => {
                    if matrix[row][col] > tall {
                        visible.insert((row, col));
                        tallest = Some(&matrix[row][col]);
                    }
                }
            }
            if *tallest.unwrap() == TALLEST_POSSIBLE {
                break;
            }
        }
    }
    for col in (0..matrix[0].len()).rev() {
        tallest = None;
        for row in 0..matrix.len() {
            match tallest {
                None => {
                    visible.insert((row, col));
                    tallest = Some(&matrix[row][col]);
                }
                Some(&tall) => {
                    if matrix[row][col] > tall {
                        visible.insert((row, col));
                        tallest = Some(&matrix[row][col]);
                    }
                }
            }
            if *tallest.unwrap() == TALLEST_POSSIBLE {
                break;
            }
        }
        tallest = None;
        for row in (0..matrix.len()).rev() {
            match tallest {
                None => {
                    visible.insert((row, col));
                    tallest = Some(&matrix[row][col]);
                }
                Some(&tall) => {
                    if matrix[row][col] > tall {
                        visible.insert((row, col));
                        tallest = Some(&matrix[row][col]);
                    }
                }
            }
            if *tallest.unwrap() == TALLEST_POSSIBLE {
                break;
            }
        }
    }
    println!("{:?}", visible.len());

}

fn create_matrix<E>(lines: impl Iterator<Item = Result<String, E>>) -> Vec<Row> {
    let mut matrix: Vec<Row> = Vec::with_capacity(128);
    for line_ in lines {
        if let Ok(line) = line_ {
            let mut row: Row = line
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect();
            row.shrink_to_fit();
            matrix.push(row);
        }
    }
    matrix.shrink_to_fit();
    matrix
}
