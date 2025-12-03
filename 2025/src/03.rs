use std::{fs::File, io::{BufReader, BufRead}, env};

const DIGITS: usize = 12;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
            
        })
        .collect::<Vec<Vec<u32>>>();

    let mut total: u64 = 0;
    for line in lines {
        let mut max: Vec<u64> = vec![0; DIGITS];
        let mut max_pos: Vec<usize> = vec![0; DIGITS];

        for i in 0..DIGITS {
            let begin = if i > 0 {max_pos[i-1] + 1} else {0};
            for (j, digit) in (&line[begin..line.len() - DIGITS + i + 1]).iter().enumerate() {
                if *digit as u64 > max[i] {
                    max[i] = *digit as u64;
                    max_pos[i] = j + begin;
                }
            }
        }
        let joltage = max.into_iter().fold(0, |acc, digit| {
            acc * 10 + digit
        });
        dbg!(joltage);
        dbg!(max_pos);
        total += joltage;
    }

    println!("Part 1: {total}");
}
