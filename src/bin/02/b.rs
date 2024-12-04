use aoc::{input_arg, read_int_rows};

#[derive(Debug, Copy, Clone)]
enum State {
    Zero,
    One(isize),
    Many(isize, bool),
}

impl State {
    fn consume(&self, n: isize, max_dist: usize) -> Option<Self> {
        use State::{Many, One, Zero};
        match *self {
            Zero => Some(One(n)),
            One(prev) => {
                if n == prev || n.abs_diff(prev) > max_dist {
                    None
                } else {
                    Some(Many(n, n > prev))
                }
            }
            Many(prev, incr) => {
                if n == prev || n.abs_diff(prev) > max_dist || (n > prev) != incr {
                    None
                } else {
                    Some(Many(n, incr))
                }
            }
        }
    }
}

fn is_safe(ns: &[isize], state: State, max_dist: usize, can_skip: bool) -> bool {
    let first = match ns.first() {
        Some(n) => *n,
        None => return true,
    };
    if let Some(next_state) = state.consume(first, max_dist) {
        if is_safe(&ns[1..], next_state, max_dist, can_skip) {
            return true;
        }
    }
    if can_skip {
        is_safe(&ns[1..], state, max_dist, false)
    } else {
        false
    }
}

fn main() {
    let lists = read_int_rows(&input_arg());
    let safe = lists
        .into_iter()
        .filter(|list| is_safe(list, State::Zero, 3, true))
        .count();
    println!("There are {} safe lists", safe);
}
