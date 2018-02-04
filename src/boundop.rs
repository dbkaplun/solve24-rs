use std::fmt;

use super::{Op, Val};

#[derive(Clone, PartialEq)]
pub enum BoundOpVal {
    Val(Val),
    BoundOp(Box<BoundOp>),
}

impl BoundOpVal {
    fn eval(&self) -> Val {
        match self {
            BoundOpVal::Val(ref n) => *n,
            BoundOpVal::BoundOp(ref o) => o.eval(),
        }
    }

    fn to_prefix_notation(&self) -> String {
        match self {
            BoundOpVal::Val(ref n) => n.to_string(),
            BoundOpVal::BoundOp(ref o) => format!("({})", o.to_prefix_notation()),
        }
    }

    fn to_infix_notation(&self) -> String {
        match self {
            BoundOpVal::Val(ref n) => n.to_string(),
            BoundOpVal::BoundOp(ref o) => format!("({})", o.to_infix_notation()),
        }
    }

    fn to_postfix_notation(&self) -> String {
        match self {
            BoundOpVal::Val(ref n) => n.to_string(),
            BoundOpVal::BoundOp(ref o) => format!("({})", o.to_postfix_notation()),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct BoundOp {
    pub left: BoundOpVal,
    pub right: BoundOpVal,
    pub op: Op,
}

impl BoundOp {
    pub fn eval(&self) -> Val {
        (self.op.f)(self.left.eval(), self.right.eval())
    }

    pub fn to_prefix_notation(&self) -> String {
        format!(
            "{}{}{}",
            self.op.name,
            self.left.to_prefix_notation(),
            self.right.to_prefix_notation(),
        )
    }

    pub fn to_infix_notation(&self) -> String {
        format!(
            "{}{}{}",
            self.left.to_infix_notation(),
            self.op.name,
            self.right.to_infix_notation(),
        )
    }

    pub fn to_postfix_notation(&self) -> String {
        format!(
            "{}{}{}",
            self.left.to_postfix_notation(),
            self.right.to_postfix_notation(),
            self.op.name,
        )
    }
}

impl fmt::Display for BoundOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_infix_notation())
    }
}

impl fmt::Debug for BoundOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoundOp {{{}}}", self.to_string())
    }
}
