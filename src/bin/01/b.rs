use std::collections::HashMap;

use aoc::{input_arg, read_lines};

fn main() {
    let lines = read_lines(&input_arg());
    let mut ns1: Vec<u32> = Vec::new();
    let mut ns2: Vec<u32> = Vec::new();
    (ns1, ns2) = lines.fold((ns1, ns2), |(mut v1, mut v2), line| {
        let mut parts = line.split_ascii_whitespace();
        let n1 = parts.next().expect("missing first number");
        let n2 = parts.next().expect("missing second number");
        v1.push(n1.parse().expect("first part is not a number"));
        v2.push(n2.parse().expect("second part is not a number"));
        (v1, v2)
    });

    let mut right: HashMap<u32, u32> = HashMap::new();
    for n in ns2 {
        *right.entry(n).or_default() += 1;
    }
    let score = ns1.into_iter().fold(0u32, |acc, n| {
        acc + n * right.get(&n).cloned().unwrap_or_default()
    });
    println!("The sum of the similarity scores is {}", score);
}
