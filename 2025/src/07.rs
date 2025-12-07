use std::{collections::HashMap, env, fs::File, io::{BufRead, BufReader}};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum State {
    Air,
    Start,
    Splitter,
    Tachyon
}

fn propagate_dp(grid: &Vec<Vec<State>>) -> u64 {
    let start_pos = grid[0].iter().enumerate()
        .fold(0usize, |acc, (i, s)| if *s == State::Start { i } else { acc });
    let mut results: Vec<Vec<u64>> = vec![vec![0; grid[0].len()]; grid.len()];
    results[grid.len() - 1] = vec![1; grid[0].len()];
    for i in (0..grid.len() - 1).rev() {
        for j in 0..grid[0].len() {
            results[i][j] = match grid[i][j] {
                State::Air | State::Start => results[i+1][j],
                State::Splitter => results[i+1][j-1] + results[i+1][j+1],
                _ => unreachable!()
            }
        }
    }

    results[0][start_pos]
}

fn propagate_single(grid: &Vec<Vec<State>>, level: usize, pos: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    let key = (level, pos);
    if let Some(&result) = cache.get(&key) {
        return result;
    }
    let result = if level >= grid.len() {
        1
    } else {
        if grid[level][pos] == State::Splitter {
            propagate_single(grid, level + 1, pos - 1, cache) +
            propagate_single(grid, level + 1, pos + 1, cache)
        } else {
            propagate_single(grid, level + 1, pos, cache)
        }
    };
    cache.insert(key, result);
    result
}

fn propagate(grid: &Vec<Vec<State>>, level: usize) -> u64 {
    if level >= grid.len() {
        0 } else {
        let tachyons = {
            if level > 0 { grid[level - 1].iter() } else { grid[0].iter() }
                .enumerate().fold(Vec::new(), |mut acc, (i, state)| {
                    if *state == State::Tachyon || *state == State::Start { 
                        acc.push(i);
                    }
                    acc
                })
        };
        let mut new_row = grid[level].clone();
        let mut splits = 0;
        for tachyon in tachyons {
            if new_row[tachyon] == State::Splitter {
                splits += 1;
                let (l, r) = (tachyon as i64 - 1, tachyon + 1);
                if l >= 0 {
                    new_row[l as usize] = State::Tachyon;
                }
                if r < new_row.len() {
                    new_row[r] = State::Tachyon;
                }
            } else {
                new_row[tachyon] = State::Tachyon;
            }
        }
        let grid = grid.iter().enumerate().map(|(i, r)| {
            if i == level {
                new_row.clone()
            } else {
                r.clone()
            }
        }).collect();
        splits + propagate(&grid, level + 1)
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().map(|c| {
            match c {
                '.' => State::Air,
                'S' => State::Start,
                '^' => State::Splitter,
                _ => unreachable!()
            }
        }).collect())
        .collect::<Vec<Vec<State>>>();
    println!("Part 1: {:?}", propagate(&lines, 0));
    // let start_pos = lines[0].iter().enumerate()
    //     .fold(0usize, |acc, (i, s)| if *s == State::Start { i } else { acc });
    // let mut cache = HashMap::new();
    // println!("Part 2: {:?}", propagate_single(&lines, 0, start_pos, &mut cache));
    println!("Part 2: {:?}", propagate_dp(&lines))
}
