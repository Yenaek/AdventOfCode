use std::{fs::File, io::{BufRead, BufReader}};
use itertools::Itertools;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let space = reader.lines()
                          .map(|l| l.unwrap())
                          .collect::<Vec<String>>();

    // horizontal
    let mut hcross = vec![];
    let mut vcross = vec![];
    for (i, line) in space.iter().enumerate() {
        if line.chars().all(|c| c == '.') {
            hcross.push(i);
        }
    }
    // vertical
    vcross.clear();
    for i in 0..space[0].len() {
        if space.iter().all(|l| l.chars().nth(i).unwrap() == '.') {
            vcross.push(i);
        }
    }

    let mut galaxies = vec![];
    for (i, line) in space.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut total_distance = 0;

    for (g1, g2) in galaxies.iter().tuple_combinations() {
        let hmax = std::cmp::max(g1.0, g2.0);
        let hmin = std::cmp::min(g1.0, g2.0);
        let vmax = std::cmp::max(g1.1, g2.1);
        let vmin = std::cmp::min(g1.1, g2.1);
        let hcrossings = hcross.iter()
                               .filter(|n| **n < hmax && **n > hmin)
                               .collect::<Vec<&usize>>()
                               .len();
        let vcrossings = vcross.iter()
                               .filter(|n| **n < vmax && **n > vmin)
                               .collect::<Vec<&usize>>()
                               .len();
        total_distance += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
        total_distance += hcrossings * 999999;
        total_distance += vcrossings * 999999;
    }
    println!("{total_distance}");
}
