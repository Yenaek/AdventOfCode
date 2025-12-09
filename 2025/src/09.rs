use std::{fs::File, io::{BufReader, BufRead}, env};

use itertools::izip;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let (x, y) = s.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();

    let mut max_area = 0;
    for (x1, y1) in lines.iter() {
        for (x2, y2) in lines.iter() {
            let area = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("Part 1: {max_area}");
}
