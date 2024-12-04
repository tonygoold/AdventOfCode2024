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
    ns1.sort();
    ns2.sort();
    let sum = ns1
        .into_iter()
        .zip(ns2)
        .fold(0, |acc, (n1, n2)| acc + n2.abs_diff(n1));
    println!("The sum of the sorted distances is {}", sum);
}
