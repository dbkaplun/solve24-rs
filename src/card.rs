use std::collections::VecDeque;

use super::{Ops, Val};
use super::util::{cartesian_product, permutations};
use super::boundop::{BoundOp, BoundOpVal};

pub const DEFAULT_SOLUTION: Val = 24.;

#[derive(Clone)]
pub struct Card {
    numbers: Vec<Val>,
    ops: Ops,
    solution: Val,
}

impl Card {
    pub fn new(numbers: Vec<Val>) -> Self {
        Self {
            numbers: numbers,
            ops: Ops::default(),
            solution: DEFAULT_SOLUTION,
        }
    }

    pub fn solve<'a>(&'a self) -> Box<Iterator<Item = BoundOp> + 'a> {
        Box::new(
            self.op_product()
                .flat_map(move |ops| {
                    permutations(&self.numbers).flat_map(move |numbers_perm| {
                        let mut q: VecDeque<Vec<BoundOpVal>> = vec![
                            numbers_perm
                                .into_iter()
                                .map(|&n| BoundOpVal::Val(n))
                                .collect(),
                        ].into();
                        while let Some(vals) = q.pop_front() {
                            let vals_len = vals.len();
                            if vals_len <= 1 {
                                break;
                            }
                            q.extend((0..vals_len - 1).map(|i| {
                                let mut new_vals = vals.clone();
                                new_vals.splice(
                                    i..i + 2,
                                    Some(BoundOpVal::BoundOp(Box::new(BoundOp {
                                        left: vals[i].clone(),
                                        right: vals[i + 1].clone(),
                                        op: ops.0[i].clone(),
                                    }))),
                                );
                                new_vals
                            }));
                        }
                        q.into_iter().filter_map(|val| match val[0].clone() {
                            BoundOpVal::BoundOp(box o) => Some(o),
                            _ => None,
                        })
                    })
                })
                .filter(move |bound_op| bound_op.eval() == self.solution),
        )
    }

    fn op_product<'a>(&'a self) -> Box<Iterator<Item = Ops> + 'a> {
        let ops = &self.ops.0;
        Box::new(
            cartesian_product(&vec![ops.len(); self.numbers.len() - 1][..])
                .map(move |is| Ops(is.into_iter().map(|i| ops[i].clone()).collect::<Vec<_>>())),
        )
    }
}