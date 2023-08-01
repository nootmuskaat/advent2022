use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<char>>;
type Coord = (usize, usize);
type InvalidMoves = HashMap<Coord, Vec<Instruction>>;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
    // Terminate
}

static DIRECTIONS: [Instruction; 4] = [
    Instruction::Up,
    Instruction::Down,
    Instruction::Left,
    Instruction::Right,
];

impl Instruction {
    fn from(&self, map: &Map, point: Coord) -> Option<Coord> {
        match &self {
            Instruction::Up => {
                if point.0 == 0 {
                    return None;
                }
                Some((point.0 - 1, point.1))
            }
            Instruction::Down => {
                if point.0 + 1 == map.len() {
                    return None;
                }
                Some((point.0 + 1, point.1))
            }
            Instruction::Left => {
                if point.1 == 0 {
                    return None;
                }
                Some((point.0, point.1 - 1))
            }
            Instruction::Right => {
                if point.1 + 1 == map[0].len() {
                    return None;
                }
                Some((point.0, point.1 + 1))
            }
        }
    }

    fn opposite(&self) -> Self {
        match &self {
            Instruction::Up => Instruction::Down,
            Instruction::Down => Instruction::Up,
            Instruction::Left => Instruction::Right,
            Instruction::Right => Instruction::Left,
        }
    }
}

enum RouteStatus {
    Deadend(Coord),
    Arrived(Vec<Coord>),
    Ongoing(Coord),
}

#[derive(Debug)]
struct Brain<'a> {
    deadends: HashSet<Coord>,
    invalids: InvalidMoves,
    start: Coord,
    end: Coord,
    map: &'a Map,
}

impl<'a> Brain<'a> {
    fn new(map: &'a Map) -> Self {
        let (start, end) = find_start_and_end(map);
        let invalids = check_for_invalid_moves(map);
        println!("Start = {:?} | End = {:?}", start, end);
        Brain {
            deadends: HashSet::new(),
            invalids,
            start,
            end,
            map,
        }
    }
}

pub fn day_main(filename: &str, _parts: u8) {
    let f = File::open(filename).expect("Unable to read file");
    let map: Map = read_map(f);
    let brain = Brain::new(&map);
    // println!("The brain is live! {:?}", brain);
}

fn read_map(f: File) -> Map {
    let mut map: Map = Vec::new();
    for line_ in BufReader::new(f).lines() {
        if let Ok(line) = line_ {
            let chars: Vec<char> = line.chars().collect();
            map.push(chars);
        }
    }
    map
}

fn find_start_and_end(map: &Map) -> (Coord, Coord) {
    let mut start: Coord = (0, 0);
    let mut end: Coord = (0, 0);
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &item) in row.iter().enumerate() {
            match item {
                'S' => {
                    start = (row_idx, col_idx);
                }
                'E' => {
                    end = (row_idx, col_idx);
                }
                _ => (),
            }
        }
    }
    (start, end)
}

fn check_instruction(
    instr: Instruction,
    map: &Map,
    row_idx: usize,
    col_idx: usize,
    bad_to_start: &mut Vec<Instruction>,
) {
    match instr.from(map, (row_idx, col_idx)) {
        None => bad_to_start.push(instr),
        Some((next_row, next_col)) => {
            if (map[next_row][next_col] as u8) > (map[row_idx][col_idx] as u8) + 1 {
                bad_to_start.push(instr);
            }
        }
    }
}

fn check_for_invalid_moves(map: &Map) -> InvalidMoves {
    let mut invalids: InvalidMoves = HashMap::with_capacity(map.len() * map[0].len());
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &item) in row.iter().enumerate() {
            let mut bad_to_start = Vec::new();
            for instr in DIRECTIONS {
                check_instruction(instr, map, row_idx, col_idx, &mut bad_to_start);
            }
            invalids.insert((row_idx, col_idx), bad_to_start);
        }
    }
    println!("Invalids:\n{:?}", invalids);
    invalids
}
