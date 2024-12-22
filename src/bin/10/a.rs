use std::collections::HashMap;

use aoc::{input_arg, read_uint_grid};

fn main() {
    let mut edges: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut heads: Vec<(usize, usize)> = Vec::new();
    let grid = read_uint_grid(&input_arg());
    for (row, col, &val) in grid.iter() {
        if val == 0 {
            heads.push((row, col));
        }
        for cell in grid.neighbours((row, col)) {
            if grid[cell] == val + 1 {
                edges.entry((row, col)).or_default().push(cell);
            }
        }
    }

    let scores = heads
        .into_iter()
        .map(|head| {
            let mut cells = vec![head];
            for _ in 1..10 {
                let nodes = cells.clone();
                cells.clear();
                for node in nodes {
                    if let Some(edges) = edges.get(&node) {
                        for edge in edges {
                            if !cells.contains(edge) {
                                cells.push(*edge);
                            }
                        }
                    }
                }
            }
            cells.len()
        })
        .collect::<Vec<usize>>();
    let sum = scores.into_iter().sum::<usize>();
    println!("The sum of trailhead scores is {}", sum);
}
