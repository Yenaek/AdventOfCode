use std::{cmp, env, fs::File, io::{BufRead, BufReader}, ops::{RangeInclusive}};
use fancy_regex::Regex;

fn regex_solution(lines: Vec<(u64, u64)>) {
    let re1 = Regex::new("^([0-9]+)\\1$").unwrap();
    let re2 = Regex::new("^([0-9]+)\\1+$").unwrap();
    let mut total1: u64 = 0;
    let mut total2: u64 = 0;
    for (lower, upper) in lines {
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
fn solution(lines: Vec<(u64, u64)>) {
    let ranges = lines.iter()
        .map(|(lower, upper)| lower..=upper)
        .collect::<Vec<RangeInclusive<&u64>>>();
    let max_digits = lines.iter()
        .fold(0, |acc, (_, upper)| { 
            cmp::max(acc, *upper) 
        })
        .to_string().len();
    let mut total: u64 = 0;
    let mut found: Vec<u64> = vec![]; // avoid duplicates
    for digits in 1..=(max_digits / 2) {
        let min = 10u64.pow((digits - 1) as u32);
        let max = 10u64.pow(digits as u32) - 1;
        let repetitions = max_digits / digits; 
        for i in min..=max {
            let mag = 10u64.pow(digits as u32);
            for j in 2..=repetitions {
                let mut num = 0u64;
                for _ in 0..j {
                    num = num.checked_mul(mag).unwrap();
                    num = num.checked_add(i).unwrap();
                }
                if ranges.iter().fold(false, |acc, range| acc || range.contains(&&num)) {
                    if !found.contains(&num) {
                        total += num;
                        found.push(num);
                    }
                }

            }
        }
    }
    println!("Part 2: {total}");
}

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
    regex_solution(lines[0].clone());
    println!("-----");
    solution(lines[0].clone());
}
