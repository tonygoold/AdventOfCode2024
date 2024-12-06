use aoc::lattice::Lattice;
use aoc::{input_arg, read_lines};

fn parse_rel(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split("|");
    let left: usize = parts.next()?.parse().ok()?;
    let right: usize = parts.next()?.parse().ok()?;
    Some((left, right))
}

fn is_good(lattice: &Lattice<usize>, list: &[usize]) -> bool {
    list.iter().enumerate().all(|(index, left)| {
        list.iter()
            .skip(index + 1)
            .all(|right| lattice.has(left, right))
    })
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

    let good_lists = lists
        .into_iter()
        .filter(|list| is_good(&lattice, list))
        .collect::<Vec<_>>();

    let middle_sum = good_lists
        .into_iter()
        .map(|list| list[list.len() / 2])
        .sum::<usize>();
    println!("The sum of the middle of good lists is {}", middle_sum);
}
