use std::iter;

pub struct Fenwick {
    pub table: Vec<u32>,
}
impl Fenwick {
    pub fn new(zero: u32) -> Self {
        Self { table: vec![zero] }
    }
    pub fn push(&mut self, x: u32) {
        let n = self.table.len();
        let lsb_n = lsb(n);
        let x = x + iter::successors(Some(1), |&d| Some(2 * d))
            .take_while(|&d| d != lsb_n)
            .map(|i| self.table[n - i])
            .sum::<u32>();
        self.table.push(x);
    }
    pub fn from_slice(src: &[u32]) -> Self {
        let mut table = vec![0; src.len() + 1];
        table[1..].copy_from_slice(src);
        let n = table.len();
        (1..n)
            .map(|i| (i, i + lsb(i)))
            .filter(|&(_, j)| j < n)
            .for_each(|(i, j)| table[j] += table[i]);
        Self { table }
    }
    pub fn prefix_sum(&self, i: usize) -> u32 {
        iter::successors(Some(i), |&i| Some(i - lsb(i)))
            .take_while(|&i| i != 0)
            .map(|i| self.table[i])
            .sum()
    }
    pub fn add(&mut self, i: usize, x: u32) {
        let n = self.table.len();
        iter::successors(Some(i + 1), |&i| Some(i + lsb(i)))
            .take_while(|&i| i < n)
            .for_each(|i| self.table[i] += x)
    }
    pub fn upper_bound(&self, x: &u32) -> usize {
        let mut d = self.table.len().next_power_of_two() / 2;
        let mut j = 0;
        let mut now = 0;
        while d != 0 {
            if d + j < self.table.len() {
                let next = now + self.table[d + j];
                if &next <= x {
                    now = next;
                    j += d;
                }
            }
            d /= 2;
        }
        j
    }
}
#[inline]
fn lsb(i: usize) -> usize {
    i & !(i.saturating_sub(1))
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
    const VALUE_LIMIT: u32 = 100;

    fn gen_value(rng: &mut StdRng) -> u32 {
        rng.gen_range(VALUE_MININUM, VALUE_LIMIT)
    }

    fn gen_index(rng: &mut StdRng, n: usize) -> usize {
        rng.gen_range(0, n)
    }

    #[test]
    fn test_hand() {
        assert_eq!(2 + 2, 4);
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        for _ in 0..TEST_COUNT {
            let mut a = {
                let n = rng.gen_range(6, 20);
                iter::repeat_with(|| gen_value(&mut rng))
                    .take(n)
                    .collect::<Vec<_>>()
            };
            let mut fenwick = Fenwick::from_slice(&a);

            println!("CREATED AN INSTANCE: a = {:?}", &a);

            for _ in 0..QUERY_COUNT {
                match rng.gen_range(0, 100) {
                    // push
                    0..=19 => {
                        let x = gen_value(&mut rng);
                        a.push(x);
                        fenwick.push(x);
                        println!("Push ( x = {} )", x);
                    }
                    // prefix_sum
                    20..=39 => {
                        let i = gen_index(&mut rng, a.len());
                        let expected = a[..i].iter().sum::<u32>();
                        let result = fenwick.prefix_sum(i);
                        println!(
                            "Prefix sum ( i = {} ) -> ( expected = {}, result = {}), a = {:?}",
                            i, expected, result, &a
                        );
                        assert_eq!(expected, result);
                    }
                    // add
                    40..=79 => {
                        let i = gen_index(&mut rng, a.len());
                        let x = gen_value(&mut rng);
                        println!("Add ( i = {}, x = {}) )", i, x);
                        a[i] += x;
                        fenwick.add(i, x);
                    }
                    // upper_bound
                    80..=99 => {
                        let x = rng.gen_range(
                            VALUE_MININUM * (a.len() / 2) as u32,
                            VALUE_LIMIT * (a.len() / 2) as u32,
                        );
                        let mut b = vec![0; a.len() + 1];
                        for (i, &x) in a.iter().enumerate() {
                            b[i + 1] = b[i] + x;
                        }
                        let expected = b.upper_bound(&x) - 1;
                        let result = fenwick.upper_bound(&x);
                        println!(
                            "Upper bound ( x = {} ) -> ( expected = {}, result = {} ), a = {:?}",
                            x, expected, result, &a
                        );
                        assert_eq!(expected, result);
                    }
                    100..=std::u32::MAX => unreachable!(),
                }
            }
            println!();
        }
    }
}
