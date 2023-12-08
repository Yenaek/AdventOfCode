use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut total = 0;

    let mut card_amounts = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let (winning, have) = line.split_once(":")
                                  .unwrap().1
                                  .split_once("|")
                                  .unwrap();
        // me when i cant map onto a tuple
        let winning: Vec<u32> = winning.split(" ")
                                       .filter(|s| !s.is_empty())
                                       .map(|n| n.parse::<u32>()
                                                 .unwrap())
                                       .collect();
        let have: Vec<u32> = have.split(" ")
                                 .filter(|s| !s.is_empty())
                                 .map(|n| n.parse::<u32>()
                                           .unwrap())
                                 .collect();

        let mut cnt = 0;
        for n in have.iter() {
            if winning.contains(n) {
                cnt += 1;
                card_amounts[i + cnt] += card_amounts[i];
            }
        }
        total += if cnt == 0 { 0 } else { 2u32.pow(cnt as u32 -1) };
    }
    println!("1: {total}");
    println!("2: {}", card_amounts.iter().sum::<u32>());
}
