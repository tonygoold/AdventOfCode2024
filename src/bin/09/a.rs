use std::cmp::Ordering;

use aoc::{input_arg, read_line};

#[derive(Clone, Copy)]
struct RunLen {
    id: Option<usize>,
    len: usize,
}

impl RunLen {
    pub fn new(id: Option<usize>, len: usize) -> Self {
        Self { id, len }
    }
}

fn main() {
    let input = read_line(&input_arg());
    let mut initial: Vec<RunLen> = Vec::with_capacity(input.len());
    let mut id = 0;
    let mut blank = false;
    for c in input.chars() {
        let len = c.to_digit(10).expect("Non-digit in input") as usize;
        if blank {
            initial.push(RunLen::new(None, len));
            blank = false;
        } else {
            initial.push(RunLen::new(Some(id), len));
            blank = true;
            id += 1;
        }
    }

    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut left = 0usize;
    let mut right = initial.len() - 1;
    while left < right {
        let mut block = initial[left];
        if let Some(id) = block.id {
            result.push((id, block.len));
            left += 1;
            continue;
        }
        let mut moving = initial[right];
        let id = match moving.id {
            Some(val) => val,
            None => {
                right -= 1;
                continue;
            }
        };
        match block.len.cmp(&moving.len) {
            Ordering::Less => {
                result.push((id, block.len));
                moving.len -= block.len;
                initial[right] = moving;
                left += 1;
            }
            Ordering::Equal => {
                result.push((id, moving.len));
                left += 1;
                right -= 1;
            }
            Ordering::Greater => {
                result.push((id, moving.len));
                block.len -= moving.len;
                initial[left] = block;
                right -= 1;
            }
        }
    }
    if left == right {
        let last = initial[right];
        if let Some(id) = last.id {
            result.push((id, last.len));
        }
    }

    let mut index = 0usize;
    let checksum: usize = result
        .into_iter()
        .map(|(id, len)| {
            let mut prod = 0usize;
            for i in index..index + len {
                prod += id * i;
            }
            index += len;
            prod
        })
        .sum();
    println!("The checksum is {}", checksum);
}
