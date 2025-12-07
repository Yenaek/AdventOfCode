use std::{env, fs::File, io::{BufRead, BufReader}, iter::zip};
use itertools::Itertools;
use nalgebra::DMatrix;

#[derive(Debug)]
enum Op {
    Add,
    Multiply
}

fn part1(lines: &Vec<String>) {
    let lines = lines.iter()
        .map(|s| {
            s.split_whitespace()
                .map(|w| w.to_owned())
                .collect()
        })
        .collect::<Vec<Vec<String>>>();
    let (nums_str, ops_str) = lines.split_at(lines.len() - 1);

    let nums = nums_str.iter().map(|l| {
        l.iter().map(|s| s.parse::<i64>().unwrap()).collect()
    }).collect::<Vec<Vec<i64>>>();
    let ops = ops_str[0].iter().map(|s| {
        match s.as_str() {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => unreachable!()
        }
    }).collect::<Vec<Op>>();

    let mut answers = vec![];
    for (i, op) in ops.iter().enumerate() {
        let total = match op {
            Op::Add => nums.iter().fold(0, |acc, v| {
                acc + v[i]
            }),
            Op::Multiply => nums.iter().fold(1, |acc, v| {
                acc * v[i]
            }),
        };
        answers.push(total);
    }
    println!("Part 1: {}", answers.iter().sum::<i64>());
}

fn part2(lines: &Vec<String>) {
    let (nums_str, ops_str) = lines.split_at(lines.len() - 1);
    let lines = nums_str.iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let chars: Vec<char> = lines.iter().flatten().map(|c| c.to_owned()).collect();
    let values = DMatrix::from_row_slice(
        lines.len(), 
        lines[0].len(), 
        &chars)
        .transpose()
        .row_iter()
        .map(|r| r.into_iter().collect::<String>())
        .chunk_by(|s| !s.trim().is_empty())
        .into_iter()
        .filter(|(k, _)| *k)
        .map(|(_, g)| g.map(|i| i.trim().parse::<i64>().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>();
    let ops = ops_str[0].split_whitespace().map(|s| {
        match s {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => unreachable!()
        }
    }).collect::<Vec<Op>>();
    let mut answers = vec![];
    for (nums, op) in zip(values, ops) {
        let total = match op {
            Op::Add => nums.iter().fold(0, |acc, v| {
                acc + v
            }),
            Op::Multiply => nums.iter().fold(1, |acc, v| {
                acc * v
            }),
        };
        answers.push(total);
    }
    println!("Part 2: {}", answers.iter().sum::<i64>());

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    part1(&lines);
    part2(&lines);
}
