use std::collections::{HashMap, HashSet};

use aoc::{input_arg, read_char_grid};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.row + other.row, self.col + other.col)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self::new(self.row - other.row, self.col - other.col)
    }

    pub fn in_bounds(&self, lower: Coord, upper: Coord) -> bool {
        self.row >= lower.row
            && self.row < upper.row
            && self.col >= lower.col
            && self.col < upper.col
    }
}

impl Default for Coord {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

fn main() {
    let grid = read_char_grid(&input_arg());
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
    for (row, col, &c) in grid.iter() {
        if c == '.' {
            continue;
        }
        antennas
            .entry(c)
            .or_default()
            .push(Coord::new(row as isize, col as isize));
    }

    let (lower, upper) = {
        let size = grid.size();
        (
            Coord::default(),
            Coord::new(size.0 as isize, size.1 as isize),
        )
    };
    let mut anti_nodes: HashSet<Coord> = HashSet::new();
    for coords in antennas.values() {
        for (i, c1) in coords.iter().enumerate() {
            for c2 in coords.iter().skip(i + 1) {
                let delta = c2.sub(c1);
                let mut c = *c2;
                while c.in_bounds(lower, upper) {
                    anti_nodes.insert(c);
                    c = c.add(&delta);
                }
                c = *c2;
                while c.in_bounds(lower, upper) {
                    anti_nodes.insert(c);
                    c = c.sub(&delta);
                }
            }
        }
    }

    let num_anti_nodes = anti_nodes.len();
    println!("There are {} anti nodes", num_anti_nodes);
}
