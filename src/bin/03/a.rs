use aoc::{input_arg, read_all};
use regex::Regex;

fn main() {
    let input = read_all(&input_arg());
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("unable to compile regex");
    let matches = re.captures_iter(&input);
    let product_sum = matches
        .map(|cap| {
            let x: isize = cap[1].parse().expect("left is not a number");
            let y: isize = cap[2].parse().expect("right is not a number");
            x * y
        })
        .sum::<isize>();
    println!("The sum of multiplications is {}", product_sum)
}
