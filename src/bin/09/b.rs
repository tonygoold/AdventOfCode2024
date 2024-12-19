use aoc::runlen::RLArray;
use aoc::{input_arg, read_line};

fn main() {
    let input = read_line(&input_arg());
    let mut blocks = RLArray::<Option<usize>>::with_capacity(input.len());
    let mut id = 0;
    let mut blank = false;
    for c in input.chars() {
        let len = c.to_digit(10).expect("Non-digit in input") as usize;
        if blank {
            blocks.push((None, len));
            blank = false;
        } else {
            blocks.push((Some(id), len));
            blank = true;
            id += 1;
        }
    }

    id -= 1;
    while id > 0 {
        let mut right = blocks.len() - 1;
        while right > 0 && blocks[right].0 != Some(id) {
            right -= 1;
        }
        if right == 0 {
            break;
        }

        let block = blocks[right];
        let mut space_index = 0;
        while space_index < right {
            let space = blocks[space_index];
            if space.0.is_none() && space.1 >= block.1 {
                break;
            }
            space_index += 1;
        }
        // No sufficiently large space to use
        if space_index >= right {
            id -= 1;
            continue;
        }

        // Replace previous position with blank space
        let _ = blocks.replace(right, (None, block.1)) - 1;

        let mut space = blocks[space_index];
        if space.1 == block.1 {
            // Perfect match
            blocks.replace(space_index, block);
        } else {
            space.1 -= block.1;
            blocks.replace(space_index, space);
            blocks.insert(space_index, block);
        }
        id -= 1;
    }

    let mut index = 0usize;
    let checksum: usize = blocks
        .into_iter()
        .map(|(id, len)| {
            let mut prod = 0usize;
            if let Some(id) = id {
                for i in index..index + len {
                    prod += id * i;
                }
            }
            index += len;
            prod
        })
        .sum();
    println!("The checksum is {}", checksum);
}
