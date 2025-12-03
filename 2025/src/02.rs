use std::{fs::File, io::{BufReader, BufRead}, env};
use fancy_regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| {
            s.split(",")
                .map(|range_str| {
                    let (lower_str, upper_str) = range_str.split_once("-").unwrap();
                    let lower = lower_str.parse::<u64>().unwrap();
                    let upper = upper_str.parse::<u64>().unwrap();
                    (lower, upper)
                })
                .collect::<Vec<(u64, u64)>>()
        })
        .collect::<Vec<Vec<(u64, u64)>>>();
    let re1 = Regex::new("^([0-9]+)\\1$").unwrap();
    let re2 = Regex::new("^([0-9]+)\\1+$").unwrap();
    let mut total1: u64 = 0;
    let mut total2: u64 = 0;
    for (lower, upper) in lines[0].clone() {
        for i in lower..=upper {
            if re1.is_match(&i.to_string()).unwrap() {
                total1 += i;
            }
            if re2.is_match(&i.to_string()).unwrap() {
                total2 += i;
            }
        }
    }
    println!("Part 1: {total1}");
    println!("Part 2: {total2}");
}
