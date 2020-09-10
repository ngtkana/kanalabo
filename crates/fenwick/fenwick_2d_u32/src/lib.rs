#[macro_use]
extern crate dbg;
use gridtools::exact_size_of_grid;
use std::ops;

pub struct Fenwick2d {
    pub table: Vec<Vec<u32>>,
}
impl Fenwick2d {
    pub fn from_slice_vec(src: &[Vec<u32>]) -> Self {
        let (h, w) = exact_size_of_grid(src);
        let mut table = vec![vec![0; w + 1]; h + 1];
        for i in 1..=h {
            for j in 1..=w {
                table[i][j] += src[i - 1][j - 1];
                let next_j = j + lsb(j);
                if next_j <= w {
                    let x = table[i][j];
                    table[i][next_j] += x;
                }
            }
        }
        for i in 1..=h {
            for j in 1..=w {
                let next_i = i + lsb(i);
                if next_i <= h {
                    let x = table[i][j];
                    table[next_i][j] += x;
                }
            }
        }
        Self { table }
    }
    pub fn double_prefix_sum(&self, mut i: usize, j: usize) -> u32 {
        let mut res = 0;
        while i != 0 {
            let mut j = j;
            while j != 0 {
                res += self.table[i][j];
                j -= lsb(j);
            }
            i -= lsb(i);
        }
        res
    }
    pub fn add(&mut self, mut i: usize, mut j: usize, x: u32) {
        let (h, w) = exact_size_of_grid(&self.table);
        i += 1;
        j += 1;
        while i < h {
            let mut j = j;
            while j < w {
                self.table[i][j] += x;
                j += lsb(j);
            }
            i += lsb(i);
        }
    }
    pub fn horizontal_upper_bound(&self, i: usize, x: &u32) -> usize {
        let table_width = exact_size_of_grid(&self.table).1;
        let mut d = table_width.next_power_of_two() / 2;
        let mut j = 0;
        let mut now = 0;
        while d != 0 {
            if j + d < table_width {
                let next = now + self.i_prefix_sum_j_raw_element(i, j + d);
                if &next <= x {
                    j += d;
                    now = next;
                }
            }
            d /= 2;
        }
        j
    }
    fn i_prefix_sum_j_raw_element(&self, mut i: usize, j: usize) -> u32 {
        let mut res = 0;
        while i != 0 {
            res += self.table[i][j];
            i -= lsb(i);
        }
        res
    }
}
#[inline]
fn lsb(i: usize) -> usize {
    let i = i as isize;
    (i & -i) as usize
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
            let mut table = {
                let h = rng.gen_range(6, 20);
                let w = rng.gen_range(6, 20);
                iter::repeat_with(|| {
                    iter::repeat_with(|| gen_value(&mut rng))
                        .take(w)
                        .collect::<Vec<_>>()
                })
                .take(h)
                .collect::<Vec<_>>()
            };
            let mut fenwick = Fenwick2d::from_slice_vec(&table);

            println!("CREATED AN INSTANCE: table = {:?}", &table);

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
