use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("input").unwrap(); 
    let reader = BufReader::new(file);
    let blocks = reader.lines()
                       .map(|l| l.unwrap())
                       .collect::<Vec<String>>()
                       .split(|s| s.is_empty())
                       .map(|v| v.to_owned()
                                 .iter()
                                 .map(|s| s.split(" ")
                                           .filter_map(|s| s.parse::<u64>().ok())
                                           .collect::<Vec<u64>>())
                                 .filter(|v| !v.is_empty())
                                 .collect::<Vec<Vec<u64>>>())
                       .collect::<Vec<Vec<Vec<u64>>>>();
    
    // let seeds = &blocks[0][0]; // part one
    let mut seeds: Vec<u64> = vec![];
    for i in (0..blocks[0][0].len()).step_by(2) {
        seeds.extend((blocks[0][0][i]..blocks[0][0][i]+blocks[0][0][i+1]).collect::<Vec<u64>>());
    }

    println!("Done creating seeds array");


    let mut min_location = std::u64::MAX;

    for seed in seeds {
        let mut id = seed;
        'ins: for instructions in &blocks[1..] {
            for instruction in instructions {
                let dest = instruction[0];
                let source = instruction[1];
                let range = instruction[2];
                if id >= source && id <= source + range {
                    id = dest + (id - source);
                    continue 'ins;
                }     
            }
        }
        min_location = std::cmp::min(min_location, id);
    }
    println!("{min_location}");
}
