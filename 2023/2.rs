use regex::Regex;
use std::{fs::File, io::BufReader, io::BufRead};

fn check_if_possible(v: &Vec<(u32, &str)>) -> bool {
    for (n, color) in v.iter() {
        match *color {
            "red"   => if *n > 12 { return false; } else { continue; },
            "green" => if *n > 13 { return false; } else { continue; },
            "blue"  => if *n > 14 { return false; } else { continue; },
            _ => unreachable!()
        }
    }
    return true;
}

fn simplify_tuple(v: &Vec<(u32, &str)>) -> (u32, u32, u32) {
    let mut t = (0u32,
                 0u32,
                 0u32);
    for (n, color) in v.iter() {
        match *color {
            "red"   => t.0 = *n,
            "green" => t.1 = *n,
            "blue"  => t.2 = *n,
            _ => unreachable!()
        }
    }
    return t;
}

fn max_t<T>(t1: (T, T, T), t2: (T, T, T)) -> (T, T, T)
    where T: Ord {
    return (std::cmp::max(t1.0, t2.0),
            std::cmp::max(t1.1, t2.1),
            std::cmp::max(t1.2, t2.2));
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut total_possible = 0;
    let mut total_power = 0;

    let re = Regex::new(r"[0-9]+ (red|blue|green)").unwrap();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let sets: Vec<&str> = line.split(";").collect();
        let mut max_tuple = (0u32,
                             0u32,
                             0u32);
        let mut possible = true;
        for set in sets.iter() {
            let cubes: Vec<_> = re.find_iter(set)
                                  .map(|m| match m.as_str()
                                                 .split(" ")
                                                 .collect::<Vec<&str>>().as_slice() {
                                                     &[first, second, ..] => (first.parse::<u32>().unwrap(), second),
                                                     _ => unreachable!()
                                                 })
                                  .collect();
            max_tuple = max_t(max_tuple, simplify_tuple(&cubes));
            possible = possible && check_if_possible(&cubes);
        }
        if possible {
            total_possible += i+1;
        }
        total_power += max_tuple.0 * max_tuple.1 * max_tuple.2;
    }
    println!("1: {total_possible}");
    println!("2: {total_power}");
}
