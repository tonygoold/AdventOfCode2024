use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};

use aoc::grid::Grid;
use aoc::{input_arg, read_char_grid};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn rotate_cw(self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
}

fn next_pos(g: &Grid<char>, pos: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
    let (rows, cols) = g.size();
    match dir {
        Dir::N => {
            if pos.0 > 0 {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Dir::E => {
            if pos.1 + 1 < cols {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
        Dir::S => {
            if pos.0 + 1 < rows {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        Dir::W => {
            if pos.1 > 0 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
    }
}

struct Guard {
    position: (usize, usize),
    dir: Dir,
}

impl Guard {
    pub fn new(position: (usize, usize), dir: Dir) -> Self {
        Self { position, dir }
    }

    pub fn advance(&mut self, g: &Grid<char>) -> Option<((usize, usize), Dir)> {
        let pos = next_pos(g, self.position, self.dir)?;
        if g[pos] == '#' {
            self.dir = self.dir.rotate_cw();
        } else {
            self.position = pos;
        }
        Some((self.position, self.dir))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Loop,
    Escape,
}

struct Solver<'a> {
    grid: &'a Grid<char>,
    start_pos: (usize, usize),
    visited: HashMap<(usize, usize), HashSet<Dir>>,
}

impl<'a> Solver<'a> {
    pub fn new(grid: &'a Grid<char>, start_pos: (usize, usize)) -> Self {
        Self {
            grid,
            start_pos,
            visited: HashMap::new(),
        }
    }

    pub fn solve(&mut self) -> Outcome {
        let mut guard = Guard::new(self.start_pos, Dir::N);
        while let Some((pos, dir)) = guard.advance(self.grid) {
            let entry = self.visited.entry(pos).or_default();
            if entry.contains(&dir) {
                return Outcome::Loop;
            }
            entry.insert(dir);
        }
        Outcome::Escape
    }

    pub fn visited_positions(&self) -> Keys<'_, (usize, usize), HashSet<Dir>> {
        self.visited.keys()
    }
}

fn main() {
    let mut grid = read_char_grid(&input_arg());
    let start_pos = grid
        .iter()
        .find_map(|(row, col, &c)| if c == '^' { Some((row, col)) } else { None })
        .expect("Did not find starting position");

    let mut init_solver = Solver::new(&grid, start_pos);
    let positions = match init_solver.solve() {
        Outcome::Escape => init_solver.visited_positions().cloned(),
        Outcome::Loop => panic!("Initial condition contains loops"),
    }
    .filter(|pos| pos != &start_pos)
    .collect::<Vec<_>>();

    let seen_count = positions.len() + 1;
    println!("The guard visited {} spaces", seen_count);

    let loops = positions
        .into_iter()
        .filter(|&pos| {
            grid[pos] = '#';
            let mut init_solver = Solver::new(&grid, start_pos);
            let outcome = init_solver.solve();
            grid[pos] = '.';
            outcome == Outcome::Loop
        })
        .count();
    println!("There are {} spaces which create loops", loops);
}
