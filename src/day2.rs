use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

trait Strategy {
    fn from_str(c: &str) -> Self;

    fn points_against(&self, against: &Throw) -> i32;
}

impl PartialOrd for Throw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Throw {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            match (self, other) {
                (Throw::Rock, Throw::Scissors)
                | (Throw::Scissors, Throw::Paper)
                | (Throw::Paper, Throw::Rock) => Ordering::Greater,
                _ => Ordering::Less,
            }
        }
    }
}

impl Throw {
    fn against(&self, other: &Self) -> Outcome {
        match self.cmp(other) {
            Ordering::Less => Outcome::Lose,
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Win,
        }
    }

    fn to_achieve(against: &Throw, outcome: &Outcome) -> Throw {
        match outcome {
            Outcome::Draw => against.clone(),
            Outcome::Win => match against {
                Throw::Rock => Throw::Paper,
                Throw::Paper => Throw::Scissors,
                Throw::Scissors => Throw::Rock,
            },
            Outcome::Lose => match against {
                Throw::Rock => Throw::Scissors,
                Throw::Paper => Throw::Rock,
                Throw::Scissors => Throw::Paper,
            },
        }
    }
}

impl Strategy for Throw {
    fn from_str(c: &str) -> Throw {
        match c {
            "A" | "X" => Throw::Rock,
            "B" | "Y" => Throw::Paper,
            "C" | "Z" => Throw::Scissors,
            _ => panic!(),
        }
    }

    fn points_against(&self, other: &Self) -> i32 {
        let outcome = self.against(other);
        (*self as i32) + (outcome as i32)
    }
}

impl Strategy for Outcome {
    fn from_str(c: &str) -> Outcome {
        match c {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!(),
        }
    }

    fn points_against(&self, against: &Throw) -> i32 {
        let mine = Throw::to_achieve(against, self);
        mine.points_against(against)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_throws() {
        assert_eq!(Throw::from_str("A"), Throw::Rock);
        assert_eq!(Throw::from_str("Y"), Throw::Paper);
        assert_eq!(Throw::from_str("C"), Throw::Scissors);
    }

    #[test]
    fn rock_over_scissors() {
        assert_eq!(Throw::Rock.against(&Throw::Scissors), Outcome::Win);
        assert_eq!(Throw::Rock.points_against(&Throw::Scissors), 7);
        assert_eq!(Outcome::Win.points_against(&Throw::Scissors), 7);
    }

    #[test]
    fn rock_under_paper() {
        assert_eq!(Throw::Rock.against(&Throw::Paper), Outcome::Lose);
        assert_eq!(Throw::Rock.points_against(&Throw::Paper), 1);
        assert_eq!(Outcome::Lose.points_against(&Throw::Paper), 1);
    }

    #[test]
    fn rock_opposite_rock() {
        assert_eq!(Throw::Rock.against(&Throw::Rock), Outcome::Draw);
        assert_eq!(Throw::Rock.points_against(&Throw::Rock), 4);
        assert_eq!(Outcome::Draw.points_against(&Throw::Rock), 4);
    }
}

fn strategy(part: u8, hint: &str, theirs: &Throw) -> i32 {
    match part {
        1 => Throw::from_str(hint).points_against(&theirs),
        2 => Outcome::from_str(hint).points_against(&theirs),
        _ => panic!("unimplemented part"),
    }
}

pub fn day_main(filename: &str, part: u8) {
    let f = File::open(filename).expect("couldn't open file");
    let reader = BufReader::new(f);
    let mut points = 0;
    for line in reader.lines() {
        if let Ok(items) = line {
            let (first, second) = items.split_once(" ").expect("Invalid line received");
            let theirs = Throw::from_str(first);
            points += strategy(part, second, &theirs)
        }
    }
    println!("Result is {} points", points);
}
