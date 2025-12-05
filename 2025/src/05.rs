use std::{fs::File, io::{BufReader, BufRead}, env};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    Lower,
    Upper
}

#[derive(Debug)]
struct RangeItem {
    t: Type,
    v: u64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .fold((Vec::new(), Vec::new()), |(mut ranges, mut ids), line| {
            if line.contains("-") {
                let (lower_str, upper_str) = line.split_once("-").unwrap();
                let (lower, upper) = (lower_str.parse::<u64>().unwrap(), upper_str.parse::<u64>().unwrap());
                ranges.push(lower..=upper);
            } else if !line.is_empty() {
                ids.push(line.parse::<u64>().unwrap());
            } 
            (ranges, ids)
        });
    let (ranges, ids) = lines;
    let mut total: u64 = 0;
    for id in ids {
        if ranges.iter().fold(false, |acc, range| acc || range.contains(&id)) {
            total += 1;
        }
    }

    let mut range_list: Vec<RangeItem> = ranges.iter()
        .fold(Vec::new(), |mut acc, range| {
            let (lower, upper) = (range.start(), range.end());
            acc.push( RangeItem {
                t: Type::Lower,
                v: *lower
            });
            acc.push( RangeItem {
                t: Type::Upper,
                v: *upper
            });
            acc
        });
    range_list.sort_by(|a, b| a.t.cmp(&b.t));
    range_list.sort_by(|a, b| a.v.cmp(&b.v));
    let mut cnt = 0; // state for how many lower encountered 
    let mut min: Option<u64> = None;
    let mut total2 = 0;
    for RangeItem { t, v } in &range_list {
        match t {
            Type::Lower => {
                min = match min {
                    Some(x) => Some(x),
                    None => Some(*v)
                };
                cnt += 1;
            },
            Type::Upper => {
                cnt -= 1;
                // last Upper in sequence reached
                if cnt == 0 {
                    total2 += v - min.unwrap() + 1;
                    min = None;     
                }
            }
        } 
    }
    println!("Part 1: {total}");
    println!("Part 1: {total2}");
}
