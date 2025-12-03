use std::{fs::File, io::{BufReader, BufRead}, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
}
