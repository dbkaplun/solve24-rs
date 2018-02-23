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

    pub fn explain(&self) -> (Val, Vec<String>) {
        let mut explanation = vec![];
        match self {
            BoundOp::Val(val) => (*val, explanation),
            BoundOp::BoundOp { op, l, r } => {
                let (lv, le) = l.explain();
                explanation.extend(le);
                let (rv, re) = r.explain();
                explanation.extend(re);

                let flat_bop = BoundOp::BoundOp {
                    op: op.clone(),
                    l: Box::new(BoundOp::Val(lv)),
                    r: Box::new(BoundOp::Val(rv)),
                };
                let val = flat_bop.eval();
                explanation.push(format!("{} = {}", flat_bop.to_string(), val));
                (val, explanation)
            }
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
