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
    current: Option<Moves>,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
struct Moves {
    direction: Direction,
    count: usize,
}

trait Decrement {
    fn decrement(&self) -> Self;
}

impl Decrement for Option<Moves> {
    fn decrement(&self) -> Option<Moves> {
        match self {
            Some(m) => {
                if m.count <= 1 {
                    None
                } else {
                    Some(Moves {
                        direction: m.direction,
                        count: m.count - 1,
                    })
                }
            }
            None => None,
        }
    }
}

impl State {
    fn new(lines: LineBuffer) -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            moves: lines,
            current: None,
        }
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.head.1 += 1;
            }
            Direction::Down => {
                self.head.1 -= 1;
            }
            Direction::Left => {
                self.head.0 -= 1;
            }
            Direction::Right => {
                self.head.0 += 1;
            }
        }
    }
}

fn read_line(line: &str) -> Moves {
    let mut parts = line.split_ascii_whitespace();
    let dir_str = parts.next().expect("Blank line received");
    let count_str = parts.next().expect("Line missing move count");
    let direction = Direction::from_str(dir_str);
    let count: usize = count_str.parse().expect("Move count not an integer");
    Moves { direction, count }
}

impl Iterator for State {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            let next_move = self.moves.next();
            if let Some(Ok(this_move)) = next_move {
                self.current = Some(read_line(&this_move));
            } else if let Some(Err(e)) = next_move {
                eprintln!("Unable to process line: {:?}", e);
                return None;
            } else {
                return None;
            }
        }
        // println!(
        //     "head={:?}, tail={:?}, current={:?}",
        //     self.head, self.tail, self.current
        // );
        let old_head = self.head;
        self.move_head(self.current.as_ref().unwrap().direction);
        self.current = self.current.decrement();
        if self.head.0.abs_diff(self.tail.0) > 1 || self.head.1.abs_diff(self.tail.1) > 1 {
            self.tail = old_head;
        }
        Some(self.tail)
    }
}
