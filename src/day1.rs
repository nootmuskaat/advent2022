use std::env;
use std::fs;

fn filename() -> String {
    let args: Vec<String> = env::args().collect();
    let f = args[1].clone();
    println!("Will parse file {}", f);
    f
}

fn most_calories(filepath: &str) -> Result<usize, std::io::Error> {
    let mut count: usize = 0;
    let mut max_found: usize = 0;
    fs::read_to_string(&filepath)?
        .split("\n")
        .map(|line| line.parse::<usize>())
        .for_each(|line| match line {
            Ok(i) => {
                count += i;
                max_found = max_found.max(count);
            }
            Err(_) => {
                count = 0;
            }
        });
    Ok(max_found)
}

pub fn main() {
    let f = filename();
    if let Ok(most_calories) = most_calories(&f) {
        println!("The most calories available: {}", most_calories);
    } else {
        eprintln!("Failed to read file {}", f);
    }
}
