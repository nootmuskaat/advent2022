use std::collections::HashSet;
use std::io::{BufRead, BufReader};

type Row = Vec<u8>;
type Matrix = Vec<Row>;
type Coord = (usize, usize);
type CurrentTallest = Option<u8>;
const TALLEST_POSSIBLE: u8 = 9;
enum Found {
    Nothing,
    Taller(u8, Coord),
    Tallest(Coord),
}

enum Looking<'a> {
    North(&'a Matrix),
    South(&'a Matrix),
    East(&'a Matrix),
    West(&'a Matrix),
}

impl Looking<'_> {
    fn height_at(&self, row: usize, col: usize) -> u8 {
        match self {
            Looking::East(matrix)
            | Looking::West(matrix)
            | Looking::North(matrix)
            | Looking::South(matrix) => matrix[row][col],
        }
    }

    fn visible(&self, row: usize, col: usize) -> usize {
        let mut distance: usize = 0;
        let my_height = self.height_at(row, col);
        match self {
            Looking::North(matrix) => {
                for (idx, this_row) in matrix[..row].iter().rev().enumerate() {
                    let this_height = this_row[col];
                    distance = idx + 1;
                    if this_height >= my_height {
                        break;
                    }
                }
            }
            Looking::South(matrix) => {
                for (idx, this_row) in matrix[(row + 1)..].iter().enumerate() {
                    let this_height = this_row[col];
                    distance = idx + 1;
                    if this_height >= my_height {
                        break;
                    }
                }
            }
            Looking::East(matrix) => {
                for (idx, &this_height) in matrix[row][(col + 1)..].iter().enumerate() {
                    distance = idx + 1;
                    if this_height >= my_height {
                        break;
                    }
                }
            }
            Looking::West(matrix) => {
                for (idx, &this_height) in matrix[row][0..col].iter().rev().enumerate() {
                    distance = idx + 1;
                    if this_height >= my_height {
                        break;
                    }
                }
            }
        }
        distance
    }
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

pub fn day_main(filename: &str, part: u8) {
    let f = std::fs::File::open(filename).expect("Unable to open file");
    let lines = BufReader::new(f).lines();
    let matrix = create_matrix(lines);
    // println!("{:?}", &matrix);
    match part {
        1 => part1_main(&matrix),
        2 => part2_main(&matrix),
        _ => panic!("part not implemented"),
    }
}

fn part2_main(matrix: &Vec<Vec<u8>>) {
    let mut scores: Vec<Vec<usize>> = Vec::with_capacity(matrix.len());
    for _ in 0..matrix.len() {
        scores.push(vec![1 as usize; matrix[0].len()]);
    }
    part2_run(matrix, &mut scores);
    let mut greatest: usize = 0;
    for row in scores {
        for col in row {
            greatest = greatest.max(col);
        }
    }
    println!("Greatest score: {}", greatest);
}

fn part2_run(matrix: &Vec<Vec<u8>>, scores: &mut Vec<Vec<usize>>) {
    for (row, row_of_trees) in matrix.iter().enumerate() {
        for col in 0..row_of_trees.len() {
            scores[row][col] = Looking::East(matrix).visible(row, col)
                * Looking::West(matrix).visible(row, col)
                * Looking::South(matrix).visible(row, col)
                * Looking::North(matrix).visible(row, col);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_part2_run() {
        let matrix: Vec<Vec<u8>> = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 2, 3, 2, 3, 2, 2, 1],
            vec![3, 3, 2, 1, 1, 1, 5, 1],
            vec![4, 1, 3, 3, 3, 3, 3, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
        ];
        let mut scores: Vec<Vec<usize>> = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
        ];
        part2_run(&matrix, &mut scores);

        println!("{:?}", scores);
        let expected: Vec<Vec<usize>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 8, 2, 12, 2, 1, 0],
            vec![0, 20, 4, 1, 1, 1, 24, 0],
            vec![0, 1, 4, 3, 2, 3, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(expected, scores);
    }
}

fn part1_main(matrix: &Vec<Vec<u8>>) {
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
