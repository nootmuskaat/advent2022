use std::io::{BufRead, BufReader};

pub fn day_main(filename: &str) {
    let f = std::fs::File::open(filename).expect("Couldn't open file");
    let lines = BufReader::new(f).lines();
    let mut count = 0;

    for line in lines {
        if let Ok(line) = line {
            if let Some((a, b)) = line.split_once(",") {
                if let (Some((a_start, a_end)), Some((b_start, b_end))) =
                    (a.split_once("-"), b.split_once("-"))
                {
                    if let (Ok(a_start), Ok(a_end), Ok(b_start), Ok(b_end)) = (
                        a_start.parse::<usize>(),
                        a_end.parse::<usize>(),
                        b_start.parse::<usize>(),
                        b_end.parse::<usize>(),
                    ) {
                        if identify_problematic_pairings(a_start, a_end, b_start, b_end) {
                            count += 1;
                            println!("{} - {} <==> {} - {}", a_start, a_end, b_start, b_end);
                        }
                    }
                }
            }
        }
    }
    println!("Final count is {}", count);
}

fn identify_problematic_pairings(
    a_start: usize,
    a_end: usize,
    b_start: usize,
    b_end: usize,
) -> bool {
    (a_start <= b_start && a_end >= b_end) || (a_start >= b_start && a_end <= b_end)
}
