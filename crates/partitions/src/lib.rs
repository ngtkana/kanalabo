use std::{iter, mem};
use type_traits::Ring;

type Fp = fp::F998244353;

#[derive(Debug, Clone, PartialEq)]
pub struct Partitions {
    n: usize,
    next: Option<Vec<usize>>,
}
pub fn partitions(n: usize) -> Partitions {
    Partitions {
        n,
        next: if n == 0 {
            Some(Vec::new())
        } else {
            Some(vec![n])
        },
    }
}
impl iter::Iterator for Partitions {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if let Some(next) = &self.next {
            let next = next.clone();
            mem::replace(
                &mut self.next,
                next.iter().rposition(|&x| x != 1).map(|pos| {
                    let lim = next[pos] - 1;
                    let rest = next.len() - pos;
                    let q = rest / lim;
                    let r = rest % lim;
                    next[..pos]
                        .iter()
                        .copied()
                        .chain(iter::repeat(lim).take(1 + q))
                        .chain(if r == 0 { None } else { Some(r) })
                        .collect::<Vec<_>>()
                }),
            )
        } else {
            None
        }
    }
}

pub fn partition_fn_table<T: Ring>(n: usize) -> Vec<T> {
    let mut a = vec![T::zero(); n];
    a[0] = T::one();
    for i in 0..n {
        for &sign in &[1i64, -1] {
            for (k, d) in (1..)
                .map(|k| (k * (3 * k + sign * 1) / 2) as usize)
                .take_while(|&d| d <= i)
                .enumerate()
            {
                let x = a[i - d].clone();
                if k % 2 == 0 {
                    a[i] += x;
                } else {
                    a[i] -= x;
                }
            }
        }
    }
    a
}

pub fn conjugate(src: &[usize]) -> Vec<usize> {
    if src.is_empty() {
        Vec::new()
    } else {
        let mut res = Vec::new();
        let mut i = src.len();
        for j in 0..src[0] {
            i = (1..=i).rfind(|&i| j < src[i - 1]).unwrap();
            res.push(i);
        }
        res
    }
}

pub fn hook_length_product(lambda: &[usize]) -> Fp {
    let conj = conjugate(lambda);
    let conj = &conj;
    lambda
        .iter()
        .enumerate()
        .map(|(i, &lambda_i)| {
            (0..lambda_i).map(move |j| Fp::new(((lambda_i - j) + (conj[j] - i) - 1) as i64))
        })
        .flatten()
        .product::<Fp>()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference: https://oeis.org/A000041
    const PARTITION_FN_TABLE: [i32; 50] = [
        1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792,
        1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310, 14883,
        17977, 21637, 26015, 31185, 37338, 44583, 53174, 63261, 75175, 89134, 105558, 124754,
        147273, 173525,
    ];

    #[test]
    fn test_partition_fn_table() {
        let result = partition_fn_table::<i32>(PARTITION_FN_TABLE.len());
        assert_eq!(PARTITION_FN_TABLE.to_vec(), result);
    }

    #[test]
    fn test_partitions() {
        for i in 0..20 {
            assert_eq!(PARTITION_FN_TABLE[i], partitions(i).count() as i32);
        }
    }

    #[test]
    fn test_conjugation_involutive() {
        for i in 0..15 {
            for p in partitions(i) {
                let q = conjugate(&p);
                let r = conjugate(&q);
                assert_eq!(p, r);
            }
        }
    }

    #[test]
    fn test_hook_length_product() {
        for i in 0..15 {
            let fact = (1..=i as i64).map(Fp::new).product::<Fp>();
            let sum_of_squares = partitions(i)
                .map(|p| hook_length_product(&p))
                .map(|f| fact / f)
                .map(|x| x * x)
                .sum::<Fp>();
            assert_eq!(fact, sum_of_squares);
        }
    }
}
