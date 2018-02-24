use std::collections::VecDeque;

use super::{BoundOp, Ops, Val};
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

    pub fn solve(self) -> Box<Iterator<Item = BoundOp>> {
        let solution = self.solution;
        Box::new(self.nums_ops().flat_map(move |(numbers_perm, ops)| {
            let mut q: VecDeque<Vec<BoundOp>> =
                vec![numbers_perm.into_iter().map(BoundOp::Val).collect()].into();
            while let Some(bops) = q.pop_front() {
                let new_bops_len = bops.len() - 1; // will never be negative
                if new_bops_len == 0 {
                    break;
                }
                q.extend((0..new_bops_len).map(|i| {
                    let mut new_bops = bops.clone();

                    // take two BoundOps from `bops` and combine them into one
                    new_bops.splice(
                        i..i + 2,
                        Some(BoundOp::BoundOp {
                            l: Box::new(bops[i].clone()),
                            r: Box::new(bops[i + 1].clone()),
                            // going backwards here, but no big deal
                            op: ops.0[new_bops_len - 1].clone(),
                        }),
                    );

                    new_bops
                }));
            }
            q.into_iter().filter_map(move |bops| {
                let bop = bops[0].clone();
                if eq(bop.eval(), solution) {
                    return Some(bop);
                }
                None
            })
        }))
    }

    fn nums_ops(self) -> Box<Iterator<Item = (Vec<Val>, Ops)>> {
        let nums: Vec<_> = self.clone().numbers_permutations().collect();
        let ops: Vec<_> = self.op_product().collect();
        Box::new(
            cartesian_product(&[nums.len(), ops.len()])
                .map(move |idxs| (nums[idxs[0]].clone(), ops[idxs[1]].clone())),
        )
    }

    fn op_product(self) -> Box<Iterator<Item = Ops>> {
        let numbers_len = self.numbers.len();
        if numbers_len < 1 {
            return Box::new(None.into_iter());
        }
        let ops = self.ops.0;
        Box::new(
            cartesian_product(&vec![ops.len(); numbers_len - 1])
                .map(move |idxs| Ops(idxs.into_iter().map(|i| ops[i].clone()).collect())),
        )
    }

    fn numbers_permutations(self) -> Box<Iterator<Item = Vec<Val>>> {
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
    fn test_solve_empty() {
        assert_eq!(Card::new(vec![]).solve().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_solve_works() {
        let tests = vec![vec![1., 2., 3., 4.], vec![1., 1., 4., 6.]];
        for numbers in tests {
            let card = Card::new(numbers);
            let solution = card.solution;
            let mut count = 0;
            assert!(card.solve().all(|bop| {
                count += 1;
                bop.eval() == solution
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
            let actual = Card::new(numbers)
                .solve()
                .map(|bop| bop.to_string())
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }
    }
}
