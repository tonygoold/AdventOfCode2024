use aoc::grid::Grid;
use aoc::{input_arg, read_char_grid};

#[derive(Debug, Copy, Clone)]
enum Dir {
    NE,
    NW,
    SE,
    SW,
}

fn matches(g: &Grid<char>, row: isize, col: isize, dir: Dir) -> bool {
    use Dir::{NE, NW, SE, SW};
    let (urows, ucols) = g.size();
    let rows = urows as isize;
    let cols = ucols as isize;
    let (drow, dcol): (isize, isize) = match dir {
        NE => (-1, 1),
        NW => (-1, -1),
        SE => (1, 1),
        SW => (1, -1),
    };
    let positions = vec![
        ('M', row - drow, col - dcol),
        ('A', row, col),
        ('S', row + drow, col + dcol),
    ];
    positions.into_iter().all(|(l, r, c)| {
        r >= 0 && r < rows && c >= 0 && c < cols && g[(r as usize, c as usize)] == l
    })
}

fn main() {
    let grid = read_char_grid(&input_arg());
    let matches: usize = grid
        .iter()
        .filter(|(row, col, _)| {
            let r = *row as isize;
            let c = *col as isize;
            (matches(&grid, r, c, Dir::NE) || matches(&grid, r, c, Dir::SW))
                && (matches(&grid, r, c, Dir::NW) || matches(&grid, r, c, Dir::SE))
        })
        .count();
    println!("There are {} instances of X-MAS in the puzzle", matches);
}
