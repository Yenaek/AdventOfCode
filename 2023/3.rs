use std::{fs::File, io::{BufReader, BufRead}};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let re = Regex::new(r"[0-9]+").unwrap();
    let re2 = Regex::new(r"[^0-9\.]").unwrap();
    let re3 = Regex::new(r"\*").unwrap();

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut total = 0;

    for (i, line) in lines.iter().enumerate() {
        let ranges: Vec<_> = re.find_iter(&line)
                                 .map(|m| (m.start(), m.end()))
                                 .collect();
        for (l, r) in ranges {
            let low = std::cmp::max(0, i as i32 - 1) as usize;
            let high = std::cmp::min(lines.len() as i32 - 1, i as i32 + 1) as usize;
            let mut is_part = false;
            let value = line[l..r].parse::<u32>().unwrap();

            for j in low..=high {
                let l = std::cmp::max(0, l as i32 - 1) as usize;
                let r = std::cmp::min(lines[j].len() as i32, r as i32 + 1) as usize;

                is_part = is_part || re2.is_match(&lines[j][l..r]);

                for k in re3.find_iter(&lines[j][l..r]) {
                    gears.entry((j, l + k.start())).or_insert(vec![]).push(value);
                }
            }

            if is_part {
                total += value;
            }
        }
    }
    let mut gears_total = 0;

    for gear in gears.values() {
        if gear.len() == 2 {
            gears_total += gear.iter().product::<u32>();
        }
    }
    println!("1: {total}");
    println!("2: {gears_total}");
}
