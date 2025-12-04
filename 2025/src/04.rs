use std::{fs::File, io::{BufReader, BufRead}, env};

#[derive(Debug, Clone, PartialEq)]
enum State {
    Empty,
    Paper
}

#[derive(Clone)]
struct Cell {
    state: State,
    neighbours: u8,
}

fn solution(grid: Vec<Vec<Cell>>) -> u32 {
    let mut result = grid.clone();
    for (i, line) in grid.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            match cell.state {
                State::Paper => {
                    for r in [-1, 0, 1].into_iter() {
                        for c in [-1, 0, 1].into_iter() {
                            if r == 0 && c == 0 {
                                continue;
                            }
                            let ri: usize = {
                                let t = i as i32 + r;
                                if t >= 0 && t < grid.len() as i32 {
                                    t as usize
                                } else {
                                    continue
                                }
                            };
                            let ci: usize = {
                                let t = j as i32 + c;
                                if t >= 0 && t < grid[ri].len() as i32 {
                                    t as usize
                                } else {
                                    continue
                                }
                            };
                            result[ri][ci].neighbours += 1;
                        }
                    }
                } 
                _ => ()
                    
            }
        } 
    }

    let accessible = result.iter().flatten().fold(0u32, |acc, item| {
        acc + if item.state == State::Paper && item.neighbours < 4 { 1 } else { 0 }
    });
    println!("removed: {accessible}");

    if accessible > 0 {
        let new_grid = result.clone().iter().map(|r| r.iter().map(|c| {
            Cell {
                state: if c.state == State::Paper && c.neighbours < 4 {
                    State::Empty
                } else {
                    c.state.clone()
                },
                neighbours: 0
            }
        }).collect::<Vec<Cell>>()
        ).collect::<Vec<Vec<Cell>>>();
        solution(new_grid) + accessible
    } else {
        accessible
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| s.chars()
            .map(|c| {
                Cell {
                    state: match c {
                        '.' => State::Empty,
                        '@' => State::Paper,
                        _ => unreachable!(),
                    },
                    neighbours: 0
                }
            })
            .collect::<Vec<Cell>>()
        )
        .collect::<Vec<Vec<Cell>>>();
    println!("total removed: {}", solution(lines));
}
