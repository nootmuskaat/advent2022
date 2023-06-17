use std::env;
use std::fs;

fn filename() -> String {
    let args: Vec<String> = env::args().collect();
    let f = args[1].clone();
    println!("Will parse file {}", f);
    f
}

fn count_calories_per_elf(filepath: &str) -> Result<Vec<usize>, std::io::Error> {
    let mut calories: Vec<usize> = vec![0];
    let lines: Vec<Result<usize, _>> = fs::read_to_string(&filepath)?
        .split("\n")
        .map(|x| x.parse::<usize>())
        .collect();
    for line in lines {
        let idx = calories.len() - 1;
        let last = calories[idx];
        match line {
            Ok(i) => {
                calories[idx] = last + i;
            }
            Err(_) => {
                if last != 0 {
                    calories.push(0);
                }
            }
        }
    }
    Ok(calories)
}

pub fn main() {
    let f = filename();
    let calories = count_calories_per_elf(&f);
    if let Ok(per_elf) = calories {
        let most_calories = per_elf.iter().fold(0, |max, i| max.max(*i));
        println!("The most calories available: {}", most_calories);
    } else {
        eprintln!("Failed to read file {}", f);
    }
}
