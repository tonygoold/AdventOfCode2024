use aoc::grid::Grid;
use aoc::{input_arg, read_char_grid};

#[derive(Debug, Copy, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
    NE,
    NW,
    SE,
    SW,
}

fn matches(g: &Grid<char>, row: isize, col: isize, dir: Dir) -> bool {
    use Dir::{E, N, NE, NW, S, SE, SW, W};
    let (urows, ucols) = g.size();
    let rows = urows as isize;
    let cols = ucols as isize;
    let (drow, dcol): (isize, isize) = match dir {
        N => (-1, 0),
        E => (0, 1),
        S => (1, 0),
        W => (0, -1),
        NE => (-1, 1),
        NW => (-1, -1),
        SE => (1, 1),
        SW => (1, -1),
    };
    let positions = [
        ('X', row, col),
        ('M', row + drow, col + dcol),
        ('A', row + 2 * drow, col + 2 * dcol),
        ('S', row + 3 * drow, col + 3 * dcol),
    ];
    positions.into_iter().all(|(l, r, c)| {
        r >= 0 && r < rows && c >= 0 && c < cols && g[(r as usize, c as usize)] == l
    })
}

fn main() {
    let grid = read_char_grid(&input_arg());
    let dirs = [
        Dir::N,
        Dir::E,
        Dir::S,
        Dir::W,
        Dir::NE,
        Dir::NW,
        Dir::SE,
        Dir::SW,
    ];
    let matches: usize = grid
        .iter()
        .map(|(row, col, &c)| {
            if c != 'X' {
                return 0;
            }
            dirs.iter()
                .filter(|&&dir| matches(&grid, row as isize, col as isize, dir))
                .count()
        })
        .sum();
    println!("There are {} instances of XMAS in the puzzle", matches);
}
