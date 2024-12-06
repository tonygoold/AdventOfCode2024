use regex::{CaptureMatches, Regex};

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Do,
    Dont,
    Mul(isize, isize),
}

pub struct Tokenizer<'a> {
    s: &'a str,
    re: Regex,
}

pub struct OpIter<'a> {
    caps: CaptureMatches<'a, 'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        let re = Regex::new(r"(do)\(\)|(don't)\(\)|(mul)\((\d{1,3}),(\d{1,3})\)")
            .expect("failed to compile regex");
        Self { s, re }
    }

    pub fn op_iter(&'a self) -> OpIter<'a> {
        OpIter::new(self.re.captures_iter(self.s))
    }
}

impl<'a> OpIter<'a> {
    pub fn new(caps: CaptureMatches<'a, 'a>) -> Self {
        Self { caps }
    }
}

impl Iterator for OpIter<'_> {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        let cap = self.caps.next()?;
        if cap.get(1).is_some() {
            Some(Op::Do)
        } else if cap.get(2).is_some() {
            Some(Op::Dont)
        } else {
            let x: isize = cap[4].parse().expect("x is not a number");
            let y: isize = cap[5].parse().expect("y is not a number");
            Some(Op::Mul(x, y))
        }
    }
}
