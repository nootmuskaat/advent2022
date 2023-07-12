use std::collections::HashSet;
use std::io::{BufRead, BufReader};

type Row = Vec<u8>;
type Coord = (usize, usize);
type CurrentTallest = Option<u8>;
const TALLEST_POSSIBLE: u8 = 9;
enum Found {
    Nothing,
    Taller(u8, Coord),
    Tallest(Coord),
}

macro_rules! tallest_this_direction {
    ($matrix:ident, $visible:ident, $row:ident, $col:ident, $current_tallest:ident) => {
        match check_visible(&$matrix, ($row, $col), &$current_tallest) {
            Found::Nothing => {}
            Found::Taller(n, c) => {
                $visible.insert(c);
                $current_tallest = Some(n);
            }
            Found::Tallest(c) => {
                $visible.insert(c);
                break;
            }
        }
    };
}

pub fn day_main(filename: &str, _part: u8) {
    let f = std::fs::File::open(filename).expect("Unable to open file");
    let lines = BufReader::new(f).lines();
    let matrix = create_matrix(lines);
    // println!("{:?}", &matrix);
    let mut visible: HashSet<Coord> = HashSet::with_capacity(128);
    let mut current_tallest: CurrentTallest;
    for row in 0..matrix.len() {
        current_tallest = None;
        for col in 0..matrix[row].len() {
            tallest_this_direction!(matrix, visible, row, col, current_tallest)
        }
        current_tallest = None;
        for col in (0..matrix[row].len()).rev() {
            tallest_this_direction!(matrix, visible, row, col, current_tallest)
        }
    }
    for col in (0..matrix[0].len()).rev() {
        current_tallest = None;
        for row in 0..matrix.len() {
            tallest_this_direction!(matrix, visible, row, col, current_tallest)
        }
        current_tallest = None;
        for row in (0..matrix.len()).rev() {
            tallest_this_direction!(matrix, visible, row, col, current_tallest)
        }
    }
    println!("{:?}", visible.len());
}

fn check_visible(matrix: &Vec<Row>, coord: Coord, current_tallest: &CurrentTallest) -> Found {
    let (row, col) = coord;
    if matrix[row][col] == TALLEST_POSSIBLE {
        return Found::Tallest(coord);
    }
    match *current_tallest {
        None => {
            return Found::Taller(matrix[row][col], coord);
        }
        Some(tall) => {
            if matrix[row][col] > tall {
                return Found::Taller(matrix[row][col], coord);
            }
        }
    }
    Found::Nothing
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
