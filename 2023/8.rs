use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

#[derive(Debug)]
struct Cycle {
    s: String,
    b: u64,
    n: u64,
    done: bool,
}

impl Cycle {
    fn is_done(&self) -> bool {
        self.done
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let parsed = reader.lines()
                       .map(|l| l.unwrap())
                       .collect::<Vec<String>>()
                       .split(|s| s.is_empty())
                       .map(|v| v.to_owned())
                       .collect::<Vec<Vec<String>>>();
    let command = parsed[0][0].clone();
    let nodes = parsed[1].iter()
                         .map(|s| s.split(" = ")
                                   .map(|s| s.to_owned())
                                   .collect())
                         .collect::<Vec<Vec<String>>>();
    drop(parsed);

    let mut table: HashMap<String, (String, String)> = HashMap::new();

    for node in nodes {
        let value = node[1][1..node[1].len()-1].split(", ").collect::<Vec<&str>>();
        table.insert(node[0].clone(), (value[0].to_owned(), value[1].to_owned()));
    }

    let mut cycles = table.keys()
                          .filter(|s| s.ends_with("A"))
                          .map(|s| Cycle { s: s.clone(), b: 0, n: 0, done: false })
                          .collect::<Vec<Cycle>>();
    

    let mut steps = 0;

    'repeat: loop {
        for command in command.chars() {
            for c in cycles.iter_mut() {
                if c.done {
                    continue;
                }
                if c.s.ends_with("Z") {
                    if c.b == 0 {
                        c.b = steps;
                    } else {
                        c.n = steps - c.b;
                        c.done = true;
                    }
                }
            }
            if cycles.iter()
               .map(|c| c.is_done())
               .fold(true, |a, b| a && b) {
                break 'repeat;
            }
            for c in cycles.iter_mut() {
                match command {
                    'L' => {
                        c.s = table.get(&c.s).unwrap().0.clone();
                    }
                    'R' => {
                        c.s = table.get(&c.s).unwrap().1.clone();
                    }
                    _   => unreachable!()
                }
            }
            steps += 1;
        }
    }

    println!("{cycles:?}");
}
