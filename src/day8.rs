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
}

fn part2_run(matrix: &Vec<Vec<u8>>, scores: &mut Vec<Vec<usize>>) {
    for (row, row_of_trees) in matrix.iter().enumerate() {
        for (col, tree_height) in row_of_trees.iter().enumerate() {
            if col + 1 == row_of_trees.len() {
                scores[row][col] *= 0;
                break;
            }
            for offset in 1.. {
                if col + offset == row_of_trees.len() {
                    scores[row][col] *= offset - 1;
                    break;
                } else if row_of_trees[col + offset] >= *tree_height {
                    scores[row][col] *= offset;
                    break;
                }
            }
        }
        for (col, tree_height) in row_of_trees.iter().enumerate().rev() {
            if col == 0 {
                scores[row][col] *= 0;
                break;
            }
            for offset in 1.. {
                if col + 1 - offset == 0 {
                    scores[row][col] *= offset - 1;
                    break;
                } else if row_of_trees[col - offset] >= *tree_height {
                    scores[row][col] *= offset;
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_part2_run() {
        let matrix: Vec<Vec<u8>> = vec![
            vec![1, 2, 3, 2, 3, 2, 2, 1],
            vec![3, 3, 2, 1, 1, 1, 5, 1],
            vec![4, 1, 3, 3, 3, 3, 3, 1],
        ];
        let mut scores: Vec<Vec<usize>> = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
        ];
        part2_run(&matrix, &mut scores);

        println!("{:?}", scores);
        let expected: Vec<Vec<usize>> = vec![
            vec![0, 1, 4, 1, 6, 1, 1, 0],
            vec![0, 5, 4, 1, 1, 1, 6, 0],
            vec![0, 1, 2, 1, 1, 1, 1, 0],
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
