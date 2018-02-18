use std::fmt;

use super::{Op, Val};

#[derive(Clone, PartialEq)]
pub enum BoundOp {
    Val(Val),
    BoundOp(Op, Box<BoundOp>, Box<BoundOp>),
}

impl BoundOp {
    pub fn eval(&self) -> Val {
        match self {
            BoundOp::Val(ref n) => *n,
            BoundOp::BoundOp(ref o, ref l, ref r) => (o.f)(l.eval(), r.eval()),
        }
    }

    pub fn to_prefix_notation(&self) -> String {
        match self {
            BoundOp::Val(ref n) => n.to_string(),
            BoundOp::BoundOp(ref o, ref l, ref r) => format!(
                "({}{}{})",
                o.name,
                l.to_prefix_notation(),
                r.to_prefix_notation(),
            ),
        }
    }

    pub fn to_infix_notation(&self) -> String {
        match self {
            BoundOp::Val(ref n) => n.to_string(),
            BoundOp::BoundOp(ref o, ref l, ref r) => format!(
                "({}{}{})",
                l.to_infix_notation(),
                o.name,
                r.to_infix_notation(),
            ),
        }
    }

    pub fn to_postfix_notation(&self) -> String {
        match self {
            BoundOp::Val(ref n) => n.to_string(),
            BoundOp::BoundOp(ref o, ref l, ref r) => format!(
                "({}{}{})",
                l.to_postfix_notation(),
                r.to_postfix_notation(),
                o.name,
            ),
        }
    }
}

impl fmt::Display for BoundOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_infix_notation())
    }
}

impl fmt::Debug for BoundOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoundOp {}", self.to_string())
    }
}
