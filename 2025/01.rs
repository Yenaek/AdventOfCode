use std::{fs::File, io::{BufReader, BufRead}, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let (dir, amount) = s.split_at(1);
            (dir.to_string(), amount.parse::<i32>().unwrap())
        })
        .collect::<Vec<(String, i32)>>();

    let mut pos: i32 = 50;
    let mut rest_at_zero: u32 = 0;
    let mut cross_zero: u32 = 0;
    for (dir, amount) in lines {
        let prev_pos = pos.clone();
        match dir.as_str() {
            "L" => pos -= amount,
            "R" => pos += amount,
            _ => unreachable!()
        }
        cross_zero += (pos.abs() / 100) as u32;
        if pos == 0 {
            cross_zero += 1;
        }
        match prev_pos {
            x if x > 0 => cross_zero += if pos < 0 { 1 } else { 0 },
            x if x < 0 => cross_zero += if pos > 0 { 1 } else { 0 },
            _ => ()
        }
        pos = pos % 100;
        if pos == 0 {
            rest_at_zero += 1;
        }
    }
    println!("Part 1: {rest_at_zero}");
    println!("Part 2: {cross_zero}");
}
