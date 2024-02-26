use std::{fs::File, io::{BufRead, BufReader}, vec};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap()
                       .split_whitespace()
                       .filter_map(|s| s.parse::<i32>().ok())
                       .collect::<Vec<i32>>();
        let mut lines = vec![line.clone()];
        loop {
            let mut diff = vec![];
            for (i, elem) in lines.last().unwrap()[1..].iter().enumerate() {
                diff.push(*elem - lines.last().unwrap()[i]);
            }
            // all (==0) diff
            if diff.iter().map(|e| *e == 0).fold(true, |a, b| a && b) {
                lines.push(diff);
                break;
            }
            lines.push(diff);
        }
        let mut curr = 0;
        for line in lines.iter_mut().rev() {
            // part 1
            // line.push(line.last().unwrap() + curr);
            // curr = *line.last().unwrap();
            line.insert(0, line.first().unwrap() - curr);
            curr = *line.first().unwrap();
        }
        result += curr;
    }

    println!("{result}");
}
