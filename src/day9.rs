use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Coords = (isize, isize);
type LineBuffer = Lines<BufReader<File>>;

pub fn day_main(filename: &str, part: u8) {
    let f = File::open(filename).expect("Unable to open file");
    let length: usize = match part {
        1 => 2,
        2 => 10,
        _ => panic!("Not implemented"),
    };
    let lines = BufReader::new(f).lines();
    let coords: HashSet<Coords> = State::new(length, lines).collect();
    println!("Total spaces covered: {}", coords.len());
}

struct State {
    rope: Vec<Coords>,
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
    fn new(length: usize, lines: LineBuffer) -> Self {
        Self {
            rope: vec![(0, 0); length],
            moves: lines,
            current: None,
        }
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.rope[0].1 += 1;
            }
            Direction::Down => {
                self.rope[0].1 -= 1;
            }
            Direction::Left => {
                self.rope[0].0 -= 1;
            }
            Direction::Right => {
                self.rope[0].0 += 1;
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
        // println!("{:?}", self.rope);
        // println!("instruction={:?}", self.current);
        self.move_head(self.current.as_ref().unwrap().direction);
        self.current = self.current.decrement();

        for i in 1..self.rope.len() {
            if i == self.rope.len() {
                break;
            }
            let tail = self.rope[i];
            let head = self.rope[i - 1];
            if head.0.abs_diff(tail.0) > 1 {
                // && head.1.abs_diff(tail.1) == 1 {
                let new_x = tail.0 + ((head.0 - tail.0) / 2);
                let new_y = if head.1 == tail.1 {
                    tail.1
                } else if head.1.abs_diff(tail.1) == 2 {
                    tail.1 + ((head.1 - tail.1) / 2)
                } else {
                    head.1
                };
                self.rope[i] = (new_x, new_y);
            } else if head.1.abs_diff(tail.1) > 1 {
                //&& head.0.abs_diff(tail.0) == 1 {
                let new_y = tail.1 + ((head.1 - tail.1) / 2);
                let new_x = if head.0 == tail.0 { tail.0 } else { head.0 };
                self.rope[i] = (new_x, new_y);
            } else {
                break;
            }
        }
        Some(self.rope[self.rope.len() - 1])
    }
}
