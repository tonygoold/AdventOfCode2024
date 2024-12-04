use aoc::computer::{Op, Tokenizer};
use aoc::{input_arg, read_all};

#[derive(Debug, Copy, Clone)]
struct State {
    sum: isize,
    enabled: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            sum: 0,
            enabled: true,
        }
    }
}

fn main() {
    let input = read_all(&input_arg());
    let tokenizer = Tokenizer::new(&input);
    let state = tokenizer.op_iter().fold(State::default(), |mut state, op| {
        match op {
            Op::Do => state.enabled = true,
            Op::Dont => state.enabled = false,
            Op::Mul(x, y) => {
                if state.enabled {
                    state.sum += x * y;
                }
            }
        };
        state
    });
    println!("The sum of multiplications is {}", state.sum);
}
