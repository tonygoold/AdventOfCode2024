use std::str::FromStr;

use aoc::math::remove_suffix;
use aoc::{input_arg, read_lines};

#[derive(Debug, Copy, Clone)]
enum ParseError {
    MissingValue,
    MissingOperands,
    InvalidNumber,
}

struct Entry {
    value: isize,
    operands: Vec<isize>,
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i1 = s.split(": ");
        let val_str = i1.next().ok_or(ParseError::MissingValue)?;
        let ops_str = i1.next().ok_or(ParseError::MissingOperands)?;
        let value: isize = val_str.parse().map_err(|_| ParseError::InvalidNumber)?;
        let operands = ops_str
            .split(" ")
            .map(|op| op.parse::<isize>().map_err(|_| ParseError::InvalidNumber))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { value, operands })
    }
}

impl Entry {
    pub fn has_solution(&self) -> bool {
        if self.operands.is_empty() {
            return false;
        }
        self.has_solution_at_offset(self.operands.len() - 1, self.value)
    }

    fn has_solution_at_offset(&self, offset: usize, target: isize) -> bool {
        if offset >= self.operands.len() {
            return false;
        }
        let val = self.operands[offset];
        if offset == 0 {
            return val == target;
        }
        if val <= target && self.has_solution_at_offset(offset - 1, target - val) {
            return true;
        }
        if target % val == 0 && self.has_solution_at_offset(offset - 1, target / val) {
            return true;
        }
        if let Some(prefix) = remove_suffix(target, val) {
            if self.has_solution_at_offset(offset - 1, prefix) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let entries = read_lines(&input_arg())
        .map(|line| Entry::from_str(&line))
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse entries");

    let sum = entries
        .into_iter()
        .map(|entry| if entry.has_solution() { entry.value } else { 0 })
        .sum::<isize>();
    println!("The sum of solvable entries is {}", sum);
}
