use std::fmt;

use super::Val;

pub type OpFn = fn(Val, Val) -> Val;

#[derive(Clone, PartialEq)]
pub struct Op {
    pub name: String,
    pub f: OpFn,
}

impl Op {
    pub fn new<S: Into<String>>(name: S, f: OpFn) -> Self {
        Self {
            name: name.into(),
            f: f,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Op {{{}}}", self.to_string())
    }
}

#[derive(Clone)]
pub struct Ops(pub Vec<Op>);

impl Default for Ops {
    fn default() -> Self {
        Ops(vec![
            Op::new("+", |a, b| a + b),
            Op::new("-", |a, b| a - b),
            Op::new("*", |a, b| a * b),
            Op::new("/", |a, b| a / b),
        ])
    }
}
