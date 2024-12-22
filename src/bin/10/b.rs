use aoc::grid::Grid;
use aoc::{input_arg, read_char_grid};

fn main() {
    let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let grid = read_char_grid(&input_arg());
    let (rows, cols) = grid.size();
    let mut paths = Grid::<u32>::new(rows, cols);
    for n in 0..10 {
        let c = chars[9 - n];
        let cells = grid.iter().filter(|(_, _, &val)| val == c);
        for (i, j, _) in cells {
            if n == 0 {
                paths[(i, j)] = 1;
                continue;
            }
            let mut path_sum = 0u32;
            if i > 0 && grid[(i - 1, j)] == chars[9 - n + 1] {
                path_sum += paths[(i - 1, j)];
            }
            if i + 1 < rows && grid[(i + 1, j)] == chars[9 - n + 1] {
                path_sum += paths[(i + 1, j)];
            }
            if j > 0 && grid[(i, j - 1)] == chars[9 - n + 1] {
                path_sum += paths[(i, j - 1)];
            }
            if j + 1 < cols && grid[(i, j + 1)] == chars[9 - n + 1] {
                path_sum += paths[(i, j + 1)];
            }
            paths[(i, j)] = path_sum;
        }
    }

    let scores: u32 = grid
        .iter()
        .filter_map(|(i, j, &val)| {
            if val == '0' {
                Some(paths[(i, j)])
            } else {
                None
            }
        })
        .sum();
    println!("The sum of trailhead ratings is {}", scores);
}
