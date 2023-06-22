pub fn filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    let f = args[1].clone();
    println!("Will parse file {}", f);
    f
}
