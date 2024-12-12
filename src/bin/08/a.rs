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

    pub fn extend(&self, other: &Self) -> Self {
        Self::new(2 * self.row - other.row, 2 * self.col - other.col)
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

    let (rows, cols) = {
        let size = grid.size();
        (size.0 as isize, size.1 as isize)
    };
    let mut anti_nodes: HashSet<Coord> = HashSet::new();
    for coords in antennas.values() {
        for (i, c1) in coords.iter().enumerate() {
            for c2 in coords.iter().skip(i + 1) {
                let a1 = c1.extend(c2);
                let a2 = c2.extend(c1);
                if a1.row >= 0 && a1.row < rows && a1.col >= 0 && a1.col < cols {
                    anti_nodes.insert(a1);
                }
                if a2.row >= 0 && a2.row < rows && a2.col >= 0 && a2.col < cols {
                    anti_nodes.insert(a2);
                }
            }
        }
    }

    let num_anti_nodes = anti_nodes.len();
    println!("There are {} anti nodes", num_anti_nodes);
}
