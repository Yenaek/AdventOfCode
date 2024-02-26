use std::{fs::File, io::{BufRead, BufReader}};
use kdam::tqdm;
use cached::proc_macro::cached;
use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum State {
    Reading,
    Ending,
    Waiting
}

#[cached]
fn count_possibilities(row: String, groups: Vec<usize>, state: State) -> usize {
    if row.is_empty() {
        match state {
            State::Ending|
            State::Waiting => if groups.is_empty() {
                return 1;
            } else { return 0; } // not enough groups
            State::Reading => return count_possibilities(".".to_owned(), groups, state)
        }
    } 
    match row.chars().nth(0).unwrap() {
        '.' => match state {
            State::Ending  |
            State::Waiting => count_possibilities(row[1..].to_owned(), groups, State::Waiting),
            State::Reading => { 0 } // group too short
        }
        '#' => match state {
            // beginning of a group
            State::Waiting => if groups.len() != 0 {
                if groups[0] > 1 {
                    let mut groups = groups.to_owned();
                    groups[0] -= 1;
                    count_possibilities(row[1..].to_owned(), groups, State::Reading)
                } else {
                    count_possibilities(row[1..].to_owned(), groups[1..].to_owned(), State::Ending) // group is done
                }
            } else { 0 } // too few groups
            State::Ending  => { 0 } // starting one right after ending
            State::Reading => if groups[0] > 1 {
                let mut groups = groups.to_owned();
                groups[0] -= 1;
                count_possibilities(row[1..].to_owned(), groups, State::Reading) // still more to read
            } else if groups[0] == 1 {
                count_possibilities(row[1..].to_owned(), groups[1..].to_owned(), State::Ending) // group is done
            } else { 0 } // group was too long
        }
        '?' => match state {
            State::Ending  => count_possibilities(row[1..].to_owned(), groups, State::Waiting),
            State::Waiting => { 
                count_possibilities(".".to_owned() + &row[1..], groups.clone(), state) +
                count_possibilities("#".to_owned() + &row[1..], groups.clone(), state) }
            State::Reading => count_possibilities("#".to_owned() + &row[1..], groups, state)
        }        
        _ => unimplemented!()
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines()
                      .map(|l| l.unwrap())
                      .collect::<Vec<String>>();
    let lines = lines.iter()
                     .map(|l| l.split_whitespace()
                               .collect())
                     .collect::<Vec<Vec<&str>>>();

    let rows = lines.iter()
                    .map(|v| v[0])
                    .collect::<Vec<&str>>();

    let groups = lines.iter()
                      .map(|s| s[1].split(",")
                                   .filter_map(|n| n.parse::<usize>().ok())
                                   .collect())
                      .collect::<Vec<Vec<usize>>>();
    drop(lines);

    let mut total = 0;

    for (row, group) in tqdm!(std::iter::zip(rows, groups)) {
        let row = std::iter::repeat(row).take(5).join("?");
        let group = std::iter::repeat(group).take(5).concat();
        let n = count_possibilities(row, group, State::Waiting);
        total += n;
        // println!("{row} {n}");
    }

    println!("{total}");
}
