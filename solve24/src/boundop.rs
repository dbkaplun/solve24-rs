use std::fmt;

use super::{Op, Val};

#[derive(Clone, PartialEq)]
pub enum BoundOp {
    Val(Val),
    BoundOp {
        op: Op,
        l: Box<BoundOp>,
        r: Box<BoundOp>,
    },
}

impl BoundOp {
    pub fn eval(&self) -> Val {
        match self {
            BoundOp::Val(n) => *n,
            BoundOp::BoundOp { op, l, r } => (op.f)(l.eval(), r.eval()),
        }
    }

    pub fn to_prefix_notation(&self) -> String {
        match self {
            BoundOp::Val(n) => n.to_string(),
            BoundOp::BoundOp { op, l, r } => format!(
                "({}{}{})",
                op.name,
                l.to_prefix_notation(),
                r.to_prefix_notation(),
            ),
        }
    }

    pub fn to_infix_notation(&self) -> String {
        match self {
            BoundOp::Val(n) => n.to_string(),
            BoundOp::BoundOp { op, l, r } => format!(
                "({}{}{})",
                l.to_infix_notation(),
                op.name,
                r.to_infix_notation(),
            ),
        }
    }

    pub fn to_postfix_notation(&self) -> String {
        match self {
            BoundOp::Val(n) => n.to_string(),
            BoundOp::BoundOp { op, l, r } => format!(
                "({}{}{})",
                l.to_postfix_notation(),
                r.to_postfix_notation(),
                op.name,
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
