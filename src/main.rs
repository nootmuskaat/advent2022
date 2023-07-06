pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub fn filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    let f = args[1].clone();
    println!("Will parse file {}", f);
    f
}

fn main() {
    let f = filename();
    day7::day_main(&f);
}
