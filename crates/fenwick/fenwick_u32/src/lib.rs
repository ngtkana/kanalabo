pub struct Fenwick {
    pub table: Vec<u32>,
}
impl Fenwick {
    pub fn new(zero: u32) -> Self {
        Self { table: vec![zero] }
    }
    pub fn push(&mut self, mut x: u32) {
        let n = self.table.len();
        let mut d = 1;
        let k = lsb(n);
        while d != k {
            x += self.table[n - d];
            d *= 2;
        }
        self.table.push(x);
    }
    pub fn from_slice(src: &[u32]) -> Self {
        let mut table = vec![0; src.len() + 1];
        for i in 1..table.len() {
            let x = src[i - 1];
            table[i] += x;
            let j = i + lsb(i);
            if j < table.len() {
                table[j] += table[i];
            }
        }
        Self { table }
    }
    pub fn prefix_sum(&self, mut i: usize) -> u32 {
        let mut res = 0;
        while i != 0 {
            res += self.table[i];
            i -= lsb(i);
        }
        res
    }
    pub fn add(&mut self, mut i: usize, x: u32) {
        i += 1;
        while i < self.table.len() {
            self.table[i] += x;
            i += lsb(i);
        }
    }
    pub fn upper_bound(&self, x: &u32) -> usize {
        let mut l = self.table.len().next_power_of_two() / 2;
        let mut d = l;
        let mut now = self.table[l];
        while d != 1 {
            d /= 2;
            if &now <= x {
                while d != 1 && self.table.len() <= l + d {
                    d /= 2;
                }
                if self.table.len() <= l + d {
                    break;
                }
                l += d;
                now += self.table[l];
            } else {
                now -= self.table[l];
                l -= d;
                now += self.table[l];
            }
        }
        if &now <= x {
            l
        } else {
            l - 1
        }
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
    const QUERY_COUNT: usize = 20;
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
                        let x = gen_value(&mut rng);
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
                    }
                    100..=std::u32::MAX => unreachable!(),
                }
            }
            println!();
        }
    }
}
