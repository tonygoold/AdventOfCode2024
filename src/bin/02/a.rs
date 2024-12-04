use aoc::{input_arg, read_int_rows};

struct SafeState {
    prev: Option<isize>,
    incr: Option<bool>,
    valid: bool,
}

fn is_safe(ns: &[isize], max_dist: isize) -> bool {
    let init = SafeState {
        prev: None,
        incr: None,
        valid: true,
    };
    ns.into_iter()
        .fold(init, |mut state, &n| {
            if !state.valid {
                return state;
            }

            if let Some(prev) = state.prev {
                state.valid = match state.incr {
                    Some(true) => n > prev && n <= prev + max_dist,
                    Some(false) => n < prev && n >= prev - max_dist,
                    None => {
                        if n == prev {
                            false
                        } else if n > prev {
                            state.incr = Some(true);
                            n <= prev + max_dist
                        } else {
                            state.incr = Some(false);
                            n >= prev - max_dist
                        }
                    }
                }
            }
            state.prev = Some(n);
            state
        })
        .valid
}

fn main() {
    let lists = read_int_rows(&input_arg());
    let safe = lists.into_iter().filter(|list| is_safe(&list, 3)).count();
    println!("There are {} safe lists", safe);
}
