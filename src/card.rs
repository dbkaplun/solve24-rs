use std::collections::VecDeque;

use super::{Ops, Val};
use super::util::{cartesian_product, permutations};
use super::boundop::BoundOp;

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
                    self.numbers_permutations().flat_map(move |numbers_perm| {
                        let mut q: VecDeque<Vec<BoundOp>> = vec![
                            numbers_perm.into_iter().map(|n| BoundOp::Val(n)).collect(),
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
                                    Some(BoundOp::BoundOp(
                                        ops.0[i].clone(),
                                        Box::new(vals[i].clone()),
                                        Box::new(vals[i + 1].clone()),
                                    )),
                                );
                                new_vals
                            }));
                        }
                        q.into_iter().map(|val| val[0].clone())
                    })
                })
                .filter(move |bound_op| bound_op.eval() == self.solution),
        )
    }

    fn op_product<'a>(&'a self) -> Box<Iterator<Item = Ops> + 'a> {
        let ops = &self.ops.0;
        Box::new(
            cartesian_product(&vec![ops.len(); self.numbers.len() - 1][..])
                .map(move |idxs| Ops(idxs.into_iter().map(|i| ops[i].clone()).collect::<Vec<_>>())),
        )
    }

    fn numbers_permutations<'a>(&'a self) -> Box<Iterator<Item = Vec<Val>> + 'a> {
        Box::new(
            permutations(self.numbers.len())
                .into_iter()
                .map(move |idxs| idxs.into_iter().map(|i| self.numbers[i]).collect()),
        )
    }
}
