use std::collections::VecDeque;

use super::{Ops, Val};
use super::boundop::BoundOp;
use super::util::{cartesian_product, permutations};
use super::val::eq;

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
                        let mut q: VecDeque<Vec<BoundOp>> =
                            vec![numbers_perm.into_iter().map(BoundOp::Val).collect()].into();
                        while let Some(bops) = q.pop_front() {
                            let new_bops_len = bops.len() - 1; // will never be negative
                            if new_bops_len == 0 {
                                break;
                            }
                            q.extend((0..new_bops_len).map(|i| {
                                let mut new_bops = bops.clone();

                                // take two BoundOps and combine them into one
                                new_bops.splice(
                                    i..i + 2,
                                    Some(BoundOp::BoundOp {
                                        op: ops.0[new_bops_len - 1].clone(), // going backwards here, but no big deal
                                        l: Box::new(bops[i].clone()),
                                        r: Box::new(bops[i + 1].clone()),
                                    }),
                                );

                                new_bops
                            }));
                        }
                        q.into_iter().map(|val| val[0].clone())
                    })
                })
                .filter(move |bound_op| eq(bound_op.eval(), self.solution)),
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

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_solve_works() {
        let tests = vec![
            vec![1., 2., 3., 4.],
            vec![1., 1., 4., 6.],
        ];
        for numbers in tests {
            let card = Card::new(numbers);
            let mut count = 0;
            assert!(card.solve().all(|bop| {
                count += 1;
                bop.eval() == card.solution
            }));
            assert!(count > 20);
        }
    }

    #[test]
    fn test_solve_difficult() {
        let tests = vec![
            (vec![1., 3., 4., 6.], vec!["(6/(1-(3/4)))"]),
            (vec![1., 4., 5., 6.], vec!["(4/(1-(5/6)))", "(6/((5/4)-1))"]),
        ];
        for (numbers, expected) in tests {
            let card = Card::new(numbers);
            let actual = card.solve()
                .map(|bop| bop.to_string())
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }
    }
}
