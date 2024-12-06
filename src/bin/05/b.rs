use std::cmp::Ordering;

use aoc::lattice::Lattice;
use aoc::{input_arg, read_lines};

fn parse_rel(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split("|");
    let left: usize = parts.next()?.parse().ok()?;
    let right: usize = parts.next()?.parse().ok()?;
    Some((left, right))
}

fn main() {
    let mut lattice = Lattice::new();
    let mut lines = read_lines(&input_arg());
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            if let Some((left, right)) = parse_rel(&line) {
                lattice.insert(left, right);
            } else {
                panic!("Failed to parse relation");
            }
        });

    let lists = lines
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().expect("not a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if lists.iter().all(|list| lattice.is_total_over(list)) {
        println!("All lists have a total order");
    } else {
        panic!("Some lists do not have a total order");
    }

    let middle_sum = lists
        .into_iter()
        .filter(|list| !list.is_sorted_by(|a, b| lattice.has(a, b)))
        .map(|mut list| {
            list.sort_unstable_by(|a, b| {
                if lattice.has(a, b) {
                    Ordering::Less
                } else if lattice.has(b, a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            list[list.len() / 2]
        })
        .sum::<usize>();
    println!("The sum of the middle of fixed lists is {}", middle_sum);
}
