use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod grid;

pub fn input_arg() -> String {
    env::args()
        .nth(1)
        .unwrap_or_else(|| "input/input.txt".to_string())
}

pub fn read_line(path: &str) -> String {
    read_lines(path).next().expect("No lines of input")
}

pub fn read_lines(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).expect("Unable to read input file");
    let reader = BufReader::new(f);
    reader
        .lines()
        .map(|x| x.expect("Unable to read input line"))
}

pub fn read_uints(path: &str) -> impl Iterator<Item = usize> {
    read_lines(path).map(|x| {
        x.parse::<usize>()
            .expect("Line was not an unsigned integer")
    })
}

// Reads groups of lines containing one unsigned integer each
pub fn read_uint_lists(path: &str) -> Vec<Vec<usize>> {
    let vecs: Vec<Vec<usize>> = vec![vec![]];
    read_lines(path).fold(vecs, |mut vecs, line| {
        if line.is_empty() {
            vecs.push(Vec::new());
        } else {
            let n = line.parse().expect("Line was not an unsigned integer");
            vecs.last_mut().unwrap().push(n);
        };
        vecs
    })
}

// Reads lines containing a variable number of unsigned integers
pub fn read_uint_rows(path: &str) -> Vec<Vec<usize>> {
    read_lines(path)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| {
                    n.parse::<usize>()
                        .expect("Line contained a non-unsigned integer")
                })
                .collect()
        })
        .collect()
}

// Reads lines containing a variable number of signed integers
pub fn read_int_rows(path: &str) -> Vec<Vec<isize>> {
    read_lines(path)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<isize>().expect("Line contained a non-integer"))
                .collect()
        })
        .collect()
}

pub fn read_uint_grid(path: &str) -> grid::Grid<usize> {
    let mut rows = 0;
    let mut cols = 0;
    let mut cells: Vec<usize> = Vec::new();
    read_lines(path).for_each(|line| {
        rows += 1;
        if cols == 0 {
            cols = line.len();
        } else if cols != line.len() {
            panic!("inconsistent grid width");
        }
        for c in line.chars() {
            let d = c.to_digit(10).expect("not a digit") as usize;
            cells.push(d);
        }
    });
    grid::Grid::new_with_cells(cells, rows, cols)
}

pub fn read_char_grid(path: &str) -> grid::Grid<char> {
    let mut rows = 0;
    let mut cols = 0;
    let mut cells: Vec<char> = Vec::new();
    read_lines(path).for_each(|line| {
        rows += 1;
        let mut cs: Vec<char> = line.chars().collect();
        if cols == 0 {
            cols = cs.len();
        } else if cols != cs.len() {
            panic!("inconsistent grid width");
        }
        cells.append(&mut cs);
    });
    grid::Grid::new_with_cells(cells, rows, cols)
}
