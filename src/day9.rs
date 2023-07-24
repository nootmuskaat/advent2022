use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Coords = (isize, isize);
type LineBuffer = Lines<BufReader<File>>;

pub fn day_main(filename: &str, part: u8) {
    let f = File::open(filename).expect("Unable to open file");
    match part {
        1 => part1(f),
        _ => panic!("Not implemented"),
    }
}

fn part1(f: File) {
    let lines = BufReader::new(f).lines();
    let coords: HashSet<Coords> = State::new(lines).collect();
    println!("Total spaces covered: {}", coords.len());
}

struct State {
    head: Coords,
    tail: Coords,
    moves: LineBuffer,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction received!"),
        }
    }
}

struct Moves<'a> {
    state: &'a mut State,
    direction: Direction,
    count: usize,
}

impl Iterator for Moves<'_> {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        let old_head = self.state.head;
        match self.direction {
            Direction::Up => {
                self.state.head.1 += 1;
            }
            Direction::Down => {
                self.state.head.1 -= 1;
            }
            Direction::Left => {
                self.state.head.0 -= 1;
            }
            Direction::Right => {
                self.state.head.0 += 1;
            }
        }
        if self.state.head.0.abs_diff(self.state.tail.0) > 1
            || self.state.head.1.abs_diff(self.state.tail.1) > 1
        {
            self.state.tail = old_head;
        }
        Some(self.state.tail)
    }
}

impl State {
    fn new(lines: LineBuffer) -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            moves: lines,
        }
    }

    fn evaluate_line(&mut self, line: &str) -> Moves {
        let mut parts = line.split_ascii_whitespace();
        let dir_str = parts.next().expect("Blank line received");
        let count_str = parts.next().expect("Line missing move count");
        let direction = Direction::from_str(dir_str);
        let count: usize = count_str.parse().expect("Move count not an integer");
        Moves {
            direction,
            count,
            state: self,
        }
    }
}

impl Iterator for State {
    type Item<'a> = Moves<'a>;

    fn next(&mut self) -> Option<Moves> {
        let next_move = self.moves.next();
        match next_move {
            Some(Ok(this_move)) => Some(self.evaluate_line(&this_move)),
            Some(Err(e)) => {
                eprintln!("Unable to process line: {:?}", e);
                None
            }
            None => None,
        }
    }
}
