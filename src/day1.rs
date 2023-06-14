use std::env;

pub fn filename() -> String {
    let args: Vec<String> = env::args().collect();
    let f = args[1].clone();
    f
}
