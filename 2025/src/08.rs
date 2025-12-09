use core::f64;
use std::{env, fs::File, io::{BufRead, BufReader}};

use itertools::Itertools;

#[derive(Debug)]
struct Node {
    pos: Vec<i64>
}

impl Node {
    fn dist(&self, other: &Node) -> f64 {
        (
            ((self.pos[0] - other.pos[0]) as f64).powf(2f64) +
            ((self.pos[1] - other.pos[1]) as f64).powf(2f64) +
            ((self.pos[2] - other.pos[2]) as f64).powf(2f64)
        ).sqrt()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let nodes = reader.lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let v = s.split(",")
                .map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            Node {
                pos: v
            }
        })
        .collect::<Vec<Node>>();

    let mut distances: Vec<Vec<f64>> = vec![vec![f64::INFINITY; nodes.len()]; nodes.len()];
    for (i, node) in nodes.iter().enumerate() {
        for (j, other) in nodes[(i+1)..].iter().enumerate() {
            let dist = node.dist(other);
            distances[i][i+j+1] = dist;
        }
    }

    let node_order = distances.iter().flatten().enumerate()
        .sorted_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        .filter_map(|(i, v)| {
            match *v {
                f64::INFINITY => None,
                _ => Some((i / nodes.len(), i % nodes.len()))
            }
        })
        .collect::<Vec<(usize, usize)>>();

    let mut groups: Vec<Vec<usize>> = vec![];
    for (i, j) in node_order.iter().take(1000) {
        let i_group = groups.iter().enumerate()
            .fold(None, |acc, (k, group)| match acc {
                    None => if group.contains(i) { Some(k) } else { None },
                    _ => acc
            });
        let j_group = groups.iter().enumerate()
            .fold(None, |acc, (k, group)| match acc {
                    None => if group.contains(j) { Some(k) } else { None },
                    _ => acc
            });

        match (i_group, j_group) { 
            (None, None) => groups.push(vec![*i, *j]),
            (Some(x), Some(y)) => {
                if x == y { continue; }
                let mut other = groups[y].clone();
                groups[x].append(&mut other);
                groups.remove(y);
            },
            (Some(x), None) => groups[x].push(*j),
            (None, Some(y)) => groups[y].push(*i)
        };
    }

    println!("Part 1: {}", groups.iter()
        .sorted_by_key(|v| v.len())
        .rev().take(3)
        .fold(1, |acc, v| acc * v.len()));

    let mut groups: Vec<Vec<usize>> = vec![];
    for (i, j) in node_order.iter() {
        let i_group = groups.iter().enumerate()
            .fold(None, |acc, (k, group)| match acc {
                    None => if group.contains(i) { Some(k) } else { None },
                    _ => acc
            });
        let j_group = groups.iter().enumerate()
            .fold(None, |acc, (k, group)| match acc {
                    None => if group.contains(j) { Some(k) } else { None },
                    _ => acc
            });

        match (i_group, j_group) { 
            (None, None) => groups.push(vec![*i, *j]),
            (Some(x), Some(y)) => {
                if x == y { continue; }
                let mut other = groups[y].clone();
                groups[x].append(&mut other);
                groups.remove(y);
            },
            (Some(x), None) => groups[x].push(*j),
            (None, Some(y)) => groups[y].push(*i)
        };

        if groups[0].len() == nodes.len() {
            println!("Part 2: {}", nodes[*i].pos[0] * nodes[*j].pos[0]);
            break;
        }
    }
}
