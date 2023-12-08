use std::fs;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn num_str_to_int(s: &str) -> u32 {
    match s {
        "one"   => 1,
        "two"   => 2,
        "three" => 3,
        "four"  => 4,
        "five"  => 5,
        "six"   => 6,
        "seven" => 7,
        "eight" => 8,
        "nine"  => 9,
        other   => other.parse().unwrap(),
    }
}

fn main() {
    let f = fs::File::open("input").unwrap();
    let r = io::BufReader::new(f);
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let rre = Regex::new(r"([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    let mut nums = Vec::new();

    for line in r.lines() {
        let line: String = line.unwrap();
        let rline: String = line.chars().rev().collect();
        let left_digit = num_str_to_int(re.find(&line).unwrap().as_str());
        let right_digit = num_str_to_int(rre.find(&rline).unwrap().as_str().chars().rev().collect::<String>().as_str());
        println!("{left_digit} {right_digit}");
        nums.push(left_digit * 10 + right_digit);
    }
    let sum: u32 = nums.iter().sum();
    println!("{sum}");
}
