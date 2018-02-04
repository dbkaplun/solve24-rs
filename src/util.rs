use std::iter;

pub fn permutations<'a, T: Clone + 'a, I: IntoIterator<Item = &'a T> + 'a>(
    xs: I,
) -> Box<Iterator<Item = Vec<&'a T>> + 'a> {
    let mut t = xs.into_iter();
    match t.next() {
        None => Box::new(iter::once(vec![])),
        Some(x) => Box::new(permutations(t).flat_map(move |p| {
            (0..p.len() + 1)
                .map(|i| {
                    let (l, r) = p.split_at(i);
                    [l, &[x], r].concat()
                })
                .collect::<Vec<_>>()
        })),
    }
}

pub fn cartesian_product(sizes: &[usize]) -> CartesianProduct {
    CartesianProduct {
        sizes: sizes.to_vec(),
        is: vec![0; sizes.len()],
        done: false,
    }
}

pub struct CartesianProduct {
    sizes: Vec<usize>,
    is: Vec<usize>,
    done: bool,
}

impl Iterator for CartesianProduct {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.sizes.iter().product::<usize>() == 0 {
            return None;
        }
        let res = self.is.clone();
        self.done = true;
        for (ii, i) in self.is.iter_mut().enumerate().rev() {
            *i += 1;
            if *i >= self.sizes[ii] {
                *i = 0;
            } else {
                self.done = false;
                break;
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let xs = vec![1, 2, 3];
        let ps = permutations(&xs).collect::<Vec<_>>();
        assert_eq!(
            ps,
            vec![
                vec![&1, &2, &3],
                vec![&2, &1, &3],
                vec![&2, &3, &1],
                vec![&1, &3, &2],
                vec![&3, &1, &2],
                vec![&3, &2, &1],
            ]
        );
    }
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
