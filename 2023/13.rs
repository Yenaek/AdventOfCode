use std::{fs::File, io::{BufReader, BufRead}};

fn flip(v: &Vec<String>) -> Vec<String> {
    let mut out = vec!["".to_owned(); v[0].len()];
    for inner in v {
        for (j, e) in inner.chars().enumerate() {
            out[j].push(e);
        }
    }
    out
}

fn hamming_distance(string: &str, other: &str) -> usize {
    std::iter::zip(string.chars(), other.chars())
        .map(|(c, o)| (c != o) as usize)
        .sum::<usize>()
}

fn find_mirror(map: &Vec<String>) -> Option<usize> {
    let end = map.len() - 1;
    let begin = map.len() % 2;

    for begin in (begin..end).step_by(2) {
        let middle = (end - begin)/2 + begin + 1;
        if std::iter::zip(begin..middle, (middle..=end).rev())
            .map(|(b, e)| hamming_distance(&map[b], &map[e]))
            .sum::<usize>() == 0 {
            return Some(middle);
        }
    }

    let end = end - begin;
    for begin in (1..=end).rev().step_by(2) {
        let middle = begin/2 + 1;
        if std::iter::zip(0..middle, (middle..=begin).rev())
            .map(|(b, e)| hamming_distance(&map[b], &map[e]))
            .sum::<usize>() == 0 {
            return Some(middle);
        }
    }
    None
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines()
                      .map(|l| l.unwrap())
                      .collect::<Vec<String>>()
                      .split(|s| s.is_empty())
                      .map(|v| v.to_owned())
                      .collect::<Vec<Vec<String>>>();

    let flipped = lines.iter()
                       .map(flip)
                       .collect::<Vec<Vec<String>>>();


    let hmirrors = lines.iter()
                        .filter_map(find_mirror)
                        .map(|n| n * 100)
                        .collect::<Vec<usize>>();
    let vmirrors = flipped.iter()
                          .filter_map(find_mirror)
                          .collect::<Vec<usize>>();

    println!("{}", hmirrors.iter().sum::<usize>() + vmirrors.iter().sum::<usize>());
}
