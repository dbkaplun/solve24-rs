pub fn permutations(size: usize) -> Permutations {
    Permutations {
        idxs: (0..size).collect(),
        swaps: vec![0; size],
        i: 0,
    }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

#[cfg(test)]
mod permutations_tests {
    use super::*;

    #[test]
    fn test_permutations_empty() {
        let perms = permutations(0).collect::<Vec<_>>();
        assert_eq!(perms, vec![vec![]] as Vec<Vec<usize>>)
    }

    #[test]
    fn test_permutations_nonempty() {
        let perms = permutations(3).collect::<Vec<_>>();
        assert_eq!(
            perms,
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 2, 0],
                vec![2, 1, 0],
            ]
        )
    }
}

pub fn cartesian_product(sizes: &[usize]) -> CartesianProduct {
    CartesianProduct {
        sizes: sizes.to_vec(),
        idxs: vec![0; sizes.len()],
        done: false,
    }
}

pub struct CartesianProduct {
    sizes: Vec<usize>,
    idxs: Vec<usize>,
    done: bool,
}

impl Iterator for CartesianProduct {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.sizes.iter().product::<usize>() == 0 {
            return None;
        }
        let res = self.idxs.clone();
        self.done = true;
        for (i, idx) in self.idxs.iter_mut().enumerate().rev() {
            *idx += 1;
            if *idx >= self.sizes[i] {
                *idx = 0;
            } else {
                self.done = false;
                break;
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod cartesian_product_tests {
    use super::*;

    #[test]
    fn test_cartesian_product_empty() {
        let prod = cartesian_product(&[1, 0]).collect::<Vec<_>>();
        assert_eq!(prod, vec![] as Vec<Vec<usize>>);
    }

    #[test]
    fn test_cartesian_product_nonempty() {
        let prod = cartesian_product(&[2, 3]).collect::<Vec<_>>();
        assert_eq!(
            prod,
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
            ]
        );
    }
}
