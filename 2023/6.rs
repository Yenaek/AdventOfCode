use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let parsed = reader.lines().map(|s| s.unwrap()
                                         .split_whitespace()
                                         .filter(|s| s.parse::<u64>().is_ok())
                                         .collect::<Vec<&str>>()
                                         .concat()
                                         .parse::<u64>()
                                         .unwrap())
                           .collect::<Vec<u64>>();
    let time = parsed[0];
    let distance = parsed[1];
    let mut variants = 0;

    for i in 0..=time {
        if (time-i)*i > distance {
            variants += 1;
        }
    }

    println!("{}", variants);
}
