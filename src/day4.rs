use std::io::{BufRead, BufReader};

pub fn day_main(filename: &str) {
    let f = std::fs::File::open(filename).expect("Couldn't open file");
    let lines = BufReader::new(f).lines();
    let mut count = 0;

    for line in lines {
        if let Ok(line) = line {
            if let Some(pair) = Pairing::from_str(&line) {
                if pair.one_overlaps_another() {
                    count += 1;
                    println!("Overlapping: {:?}", pair);
                }
            }
        }
    }
    println!("Final count is {}", count);
}

#[derive(Debug)]
struct Assignment {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Pairing {
    a_side: Assignment,
    b_side: Assignment,
}

impl Pairing {
    pub fn new(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> Self {
        Self {
            a_side: Assignment {
                start: a_start,
                end: a_end,
            },
            b_side: Assignment {
                start: b_start,
                end: b_end,
            },
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let (a, b) = s.split_once(",")?;
        let (a_start, a_end) = a.split_once("-")?;
        let (b_start, b_end) = b.split_once("-")?;
        if let (Ok(a_start), Ok(a_end)) = (a_start.parse::<usize>(), a_end.parse::<usize>()) {
            if let (Ok(b_start), Ok(b_end)) = (b_start.parse::<usize>(), b_end.parse::<usize>()) {
                return Some(Self::new(a_start, a_end, b_start, b_end));
            }
        }
        None
    }

    fn one_encompasses_another(&self) -> bool {
        // a encompasses b
        (self.a_side.start <= self.b_side.start && self.a_side.end >= self.b_side.end)
        // b encompasses a
            || (self.a_side.start >= self.b_side.start && self.a_side.end <= self.b_side.end)
    }

    fn one_overlaps_another(&self) -> bool {
        (self.b_side.start <= self.a_side.start && self.a_side.start <= self.b_side.end)
            || (self.b_side.start <= self.a_side.end && self.a_side.end <= self.b_side.end)
            || (self.a_side.start <= self.b_side.start && self.b_side.start <= self.a_side.end)
            || (self.a_side.start <= self.b_side.end && self.b_side.end <= self.a_side.end)
    }
}
