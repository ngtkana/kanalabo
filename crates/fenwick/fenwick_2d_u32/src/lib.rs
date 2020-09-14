use std::iter;

use gridtools::exact_size_of_grid;

pub struct Fenwick2d {
    pub table: Vec<Vec<u32>>,
}
impl Fenwick2d {
    pub fn new() -> Self {
        Self {
            table: vec![vec![0]],
        }
    }
    pub fn push(&mut self, i: usize, mut x: u32) {
        assert!(i < self.table.len());
        let i = i + 1;
        if i == 1 {
            self.table[0].push(0);
        }
        if i == self.table.len() {
            self.table.push(vec![0]);
        }
        let j = self.table[i].len();
        iter::successors(Some(lsb(i)), |&d| Some(d / 2)).for_each(|d| x += self.table[i - d][j]);
        iter::successors(Some(lsb(j)), |&e| Some(e / 2)).for_each(|e| x += self.table[i][j - e]);
        seq::cartesian_product(
            iter::successors(Some(lsb(i)), |&d| Some(d / 2)),
            iter::successors(Some(lsb(j)), |&e| Some(e / 2)),
        )
        .for_each(|(d, e)| x -= self.table[i - d][j - e]);
        self.table[i].push(x);
    }
    pub fn from_slice_vec(src: &[Vec<u32>]) -> Self {
        let (h, w) = exact_size_of_grid(src);
        let mut table = iter::once(vec![0; w + 1])
            .chain(
                src.iter()
                    .map(|v| iter::once(0).chain(v.iter().copied()).collect::<Vec<_>>()),
            )
            .collect::<Vec<_>>();
        for ((i, next_i), j) in seq::cartesian_product(
            (1..=h)
                .map(|i| (i, i + lsb(i)))
                .filter(|&(_, next_i)| next_i <= h),
            1..=w,
        ) {
            let x = table[i][j];
            table[next_i][j] += x
        }
        for ((j, next_j), i) in seq::cartesian_product(
            (1..=w)
                .map(|j| (j, j + lsb(j)))
                .filter(|&(_, next_j)| next_j <= w),
            1..=h,
        ) {
            let x = table[i][j];
            table[i][next_j] += x
        }
        Self { table }
    }
    pub fn double_prefix_sum(&self, i: usize, j: usize) -> u32 {
        seq::cartesian_product(
            iter::successors(Some(i), |&i| Some(i - lsb(i))).take_while(|&i| i != 0),
            iter::successors(Some(j), |&j| Some(j - lsb(j))).take_while(|&j| j != 0),
        )
        .map(|(i, j)| self.table[i][j])
        .sum()
    }
    pub fn add(&mut self, i: usize, j: usize, x: u32) {
        let (h, w) = exact_size_of_grid(&self.table);
        seq::cartesian_product(
            iter::successors(Some(i + 1), |&i| Some(i + lsb(i))).take_while(|&i| i < h),
            iter::successors(Some(j + 1), |&j| Some(j + lsb(j))).take_while(|&j| j < w),
        )
        .for_each(|(i, j)| self.table[i][j] += x);
    }
    pub fn horizontal_upper_bound(&self, i: usize, x: &u32) -> usize {
        let table_width = exact_size_of_grid(&self.table).1;
        let mut j = 0;
        let mut now = 0;
        for d in iter::successors(Some(table_width.next_power_of_two() / 2), |&d| Some(d / 2))
            .take_while(|&d| d != 0)
        {
            if j + d < table_width {
                let next = now + self.i_prefix_sum_j_raw_element(i, j + d);
                if &next <= x {
                    j += d;
                    now = next;
                }
            }
        }
        j
    }
    fn i_prefix_sum_j_raw_element(&self, i: usize, j: usize) -> u32 {
        iter::successors(Some(i), |&i| Some(i - lsb(i)))
            .take_while(|&j| j != 0)
            .map(|i| self.table[i][j])
            .sum()
    }
}
#[inline]
fn lsb(i: usize) -> usize {
    i & i.wrapping_neg()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use span::Span;
    use std::iter;

    const TEST_COUNT: usize = 20;
    const QUERY_COUNT: usize = 2000;
    const VALUE_MININUM: u32 = 0;
    const VALUE_LIMIT: u32 = 10;

    fn gen_value(rng: &mut StdRng) -> u32 {
        rng.gen_range(VALUE_MININUM, VALUE_LIMIT)
    }

    fn gen_index(rng: &mut StdRng, n: usize) -> usize {
        rng.gen_range(0, n)
    }

    #[test]
    fn test_hand() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        for _ in 0..TEST_COUNT {
            let h = rng.gen_range(6, 20);
            let w = rng.gen_range(6, 20);
            let mut table = {
                iter::repeat_with(|| {
                    iter::repeat_with(|| gen_value(&mut rng))
                        .take(w)
                        .collect::<Vec<_>>()
                })
                .take(h)
                .collect::<Vec<_>>()
            };
            let mut fenwick = Fenwick2d::from_slice_vec(&table);

            println!(
                "CREATED AN INSTANCE:\ntable:\n{:?}fenwick:\n{:?}",
                dbg::Tabular(&table),
                dbg::Tabular(&fenwick.table)
            );

            for _ in 0..QUERY_COUNT {
                let (h, w) = exact_size_of_grid(&table);
                match rng.gen_range(0, 100) {
                    // double prefix_sum
                    0..=39 => {
                        let i = gen_index(&mut rng, h);
                        let j = gen_index(&mut rng, w);
                        let expected = table[..i]
                            .iter()
                            .map(|row| row[..j].iter())
                            .flatten()
                            .sum::<u32>();
                        let result = fenwick.double_prefix_sum(i, j);
                        println!(
                            "Double prefix sum ( i = {}, j = {} ) -> ( expected = {}, result = {}), a = {:?}",
                            i, j, expected, result, &table
                        );
                        assert_eq!(expected, result);
                    }
                    // add
                    40..=79 => {
                        let i = gen_index(&mut rng, h);
                        let j = gen_index(&mut rng, w);
                        let x = gen_value(&mut rng);
                        println!("Add ( i = {}, j = {}, x = {})", i, j, x);
                        table[i][j] += x;
                        fenwick.add(i, j, x);
                    }
                    // horizontal_upper_bound
                    80..=99 => {
                        let i = gen_index(&mut rng, h);
                        let x = rng.gen_range(
                            VALUE_MININUM * (h * w / 16) as u32,
                            VALUE_LIMIT * (h * w / 16) as u32,
                        );
                        let mut b = vec![0; w + 1];
                        for v in &table[..i] {
                            b.iter_mut()
                                .skip(1)
                                .zip(v.iter())
                                .for_each(|(x, y)| *x += y);
                        }
                        for i in 0..w {
                            b[i + 1] += b[i];
                        }
                        let expected = b.upper_bound(&x) - 1;
                        let result = fenwick.horizontal_upper_bound(i, &x);
                        println!("Horizontal upper bound ( i = {}, x = {} ) -> ( expected = {}, result = {} )", i, x, expected, result);
                        assert_eq!(expected, result);
                    }
                    100..=std::u32::MAX => unreachable!(),
                }
            }
        }
    }
}
