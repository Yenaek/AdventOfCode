use std::{fs::File, io::{BufRead, BufReader}, fmt::Display, collections::{HashSet, VecDeque}};

struct Tile {
    east: bool,
    south: bool,
    west: bool,
    north: bool,
    display: char
}

#[derive(Debug)]
struct Runner {
    prev: (usize, usize),
    curr: (usize, usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

#[derive(Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

fn adjecent(pos: (usize, usize), width: i32, height: i32) -> Vec<(usize, usize, Direction)> {
    let mut output = vec![];
    for e in vec![(0, 1, Direction::South), (1, 0, Direction::East), (0, -1, Direction::North), (-1, 0, Direction::West)] {
        let pos = (pos.0 as i32, pos.1 as i32);
        if !((pos.0 + e.0 < 0 || pos.0 + e.0 >= width) ||
            (pos.1 + e.1 < 0 || pos.1 + e.1 >= height)) {
            output.push(((pos.0 + e.0) as usize,
                         (pos.1 + e.1) as usize,
                         e.2));
        }
    }
    output
}

fn runner_next(runner: &Runner, tile: &Tile) -> (usize, usize) {
    if tile.east {
        let pot = (runner.curr.0 + 1, runner.curr.1);
        if pot != runner.prev {
            return pot;
        }
    }
    if tile.south {
        let pot = (runner.curr.0, runner.curr.1 + 1);
        if pot != runner.prev {
            return pot;
        }
    }
    if tile.west {
        let pot = (runner.curr.0 - 1, runner.curr.1);
        if pot != runner.prev {
            return pot;
        }
    }
    if tile.north {
        let pot = (runner.curr.0, runner.curr.1 - 1);
        if pot != runner.prev {
            return pot;
        }
    }
    unreachable!();
}

trait TileLike {
    fn to_tile(&self) -> Tile;
}

impl TileLike for char {
    fn to_tile(&self) -> Tile {
        match self {
            '|' => Tile { east: false, south: true,  west: false, north: true,  display: '│'},
            '-' => Tile { east: true,  south: false, west: true,  north: false, display: '─'},
            'L' => Tile { east: true,  south: false, west: false, north: true,  display: '└'},
            'J' => Tile { east: false, south: false, west: true,  north: true,  display: '┘'},
            '7' => Tile { east: false, south: true,  west: true,  north: false, display: '┐'},
            'F' => Tile { east: true,  south: true,  west: false, north: false, display: '┌'},
            'S' => Tile { east: true,  south: true,  west: true,  north: true,  display: 'S'},
             _  => Tile { east: false, south: false, west: false, north: false, display: ' '},
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let mut map = reader.lines()
                    .map(|l| l.unwrap()
                              .chars()
                              .map(|c| c.to_tile())
                              .collect())
                    .collect::<Vec<Vec<Tile>>>();
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let mut loop_parts: HashSet<(usize, usize)> = HashSet::new();
    let mut outside: HashSet<(usize, usize)> = HashSet::new();
    let mut inside: HashSet<(usize, usize)> = HashSet::new();

    let mut start = (0, 0);
    'init: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j].display == 'S' {
                start = (j, i);
                break 'init;
            }
        }
    }
    let start = start;
    loop_parts.insert(start);
    let mut start_tile = Tile { east: false, south: false, west: false, north: false, display: 'S' };
    let mut runners = vec![];
    for e in adjecent(start, width, height) {
        if match e.2 {
            Direction::East  => map[e.1][e.0].west,
            Direction::South => map[e.1][e.0].north,
            Direction::West  => map[e.1][e.0].east,
            Direction::North => map[e.1][e.0].south,
        } {
            match e.2 {
                Direction::East  => start_tile.east = true,
                Direction::South => start_tile.south = true,
                Direction::West  => start_tile.west = true,
                Direction::North => start_tile.north = true,
            }
            runners.push(Runner { prev: start, curr: (e.0, e.1) });
            loop_parts.insert((e.0, e.1));
        }
    }
    map[start.1][start.0] = start_tile;
    let map = map;

    let mut distance = 1;

    // find the loop
    loop {
        for runner in runners.iter_mut() {
            let next = runner_next(runner, &map[runner.curr.1][runner.curr.0]);
            runner.prev = runner.curr;
            runner.curr = next;
            loop_parts.insert(next);
        }
        distance += 1;
        if runners.windows(2).all(|w| w[0].curr == w[1].curr) {
            break;
        }
    }

    
    for (i, v) in map.iter().enumerate() {
        let mut north = false;
        let mut south = false;
        let mut n = 0;
        for (j, t) in v.iter().enumerate() {
            if loop_parts.contains(&(j, i)) {
                north ^= t.north;
                south ^= t.south;
                if north && south {
                    n += 1;
                    north = false;
                    south = false;
                }
            } else if n%2 == 0 {
                outside.insert((j, i));
            } else {
                inside.insert((j, i));
            }
        }
    }

    for (i, v) in map.iter().enumerate() {
        for (j, t) in v.iter().enumerate() {
            if loop_parts.contains(&(j, i)) {
                print!("{t}");
            } else if outside.contains(&(j, i)) {
                print!("O");
            } else if inside.contains(&(j, i)) {
                print!("I");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
    
    println!("{distance}");
    println!("{}", inside.len());
}
