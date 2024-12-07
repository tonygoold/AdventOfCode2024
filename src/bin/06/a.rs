use aoc::grid::Grid;
use aoc::{input_arg, read_char_grid};

#[derive(Debug, Copy, Clone)]
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

    pub fn advance(&mut self, g: &Grid<char>) -> Option<(usize, usize)> {
        let pos = next_pos(g, self.position, self.dir)?;
        if g[pos] == '#' {
            self.dir = self.dir.rotate_cw();
        } else {
            self.position = pos;
        }
        Some(self.position)
    }
}

fn main() {
    let map = read_char_grid(&input_arg());
    let start_pos = map
        .iter()
        .find_map(|(row, col, &c)| if c == '^' { Some((row, col)) } else { None })
        .expect("Did not find starting position");
    let (rows, cols) = map.size();
    let mut seen: Grid<bool> = Grid::new(rows, cols);
    seen[start_pos] = true;

    let mut guard = Guard::new(start_pos, Dir::N);
    while let Some(pos) = guard.advance(&map) {
        seen[pos] = true;
    }

    let seen_count = seen.iter().filter(|(_, _, &val)| val).count();
    println!("The guard visited {} spaces", seen_count);
}
