pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    let filename = format!("static/day{}-input.txt", args.day);
    let day_main = {
        match args.day {
            1 => day1::day_main,
            2 => day2::day_main,
            3 => day3::day_main,
            4 => day4::day_main,
            5 => day5::day_main,
            6 => day6::day_main,
            7 => day7::day_main,
            8 => day8::day_main,
            _ => panic!("Not yet implemented!"),
        }
    };
    day_main(&filename, args.part);
}
