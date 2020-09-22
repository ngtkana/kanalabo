use type_traits::Assoc;

#[derive(Debug, Clone, PartialEq)]
pub struct Doubling<T: Assoc> {
    table: Vec<Vec<(usize, T)>>,
}
impl<T: Assoc> Doubling<T> {
    pub fn new(a: &[(usize, T)]) -> Self {
        let mut table = vec![a.to_vec()];
        for _ in 0..a.len().next_power_of_two().trailing_zeros() {
            let prv = table.last().unwrap();
            let mut crr = prv.clone();
            for i in 0..a.len() {
                let j = prv[i].0;
                crr[i] = (prv[j].0, prv[i].1.clone().op(prv[j].1.clone()));
            }
            table.push(crr);
        }
        Self { table }
    }
    // (距離, 終着点, 累積)
    pub fn find(
        &self,
        start: usize,
        init: T,
        mut pred: impl FnMut(usize, &T) -> bool,
    ) -> (usize, usize, T) {
        if !pred(start, &init) {
            let mut d = 0;
            let mut i = start;
            let mut value = init;
            let mut k = 1usize << (self.table.len() - 1);

            for row in self.table.iter().rev() {
                let next_i = row[i].0;
                let next_value = value.clone().op(row[i].1.clone());
                if !pred(next_i, &next_value) {
                    i = next_i;
                    value = next_value;
                    d += k;
                }
                k /= 2;
            }
            assert!(!pred(i, &value));
            (d, self.table[0][i].0, value.op(self.table[0][i].1.clone()))
        } else {
            (0, start, init)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;
    use type_traits::wrappers::Add;

    #[test]
    fn test_acl1() {
        let k = 220492538;
        let a = vec![
            4452279, 12864090, 23146757, 31318558, 133073771, 141315707, 263239555, 350278176,
            401243954, 418305779, 450172439, 560311491, 625900495, 626194585, 891960194,
        ];
        let a = iter::once(std::i64::MIN / 2)
            .chain(a)
            .chain(iter::once(std::i64::MAX / 2))
            .collect::<Vec<_>>();
        let n = a.len();

        let prev = {
            let mut prev = vec![(0, Add(0)); n];
            let mut i = n - 1;
            for j in (1..n).rev() {
                while a[j] - a[i] < k {
                    i -= 1;
                }
                prev[j] = (i, Add(j));
            }
            Doubling::new(&prev)
        };
        let next = {
            let mut next = vec![(n - 1, Add(0)); n];
            let mut j = 0;
            for i in 0..n - 1 {
                while a[j] - a[i] < k {
                    j += 1;
                }
                next[i] = (j, Add(i));
            }
            Doubling::new(&next)
        };

        for (l, r, expected) in
            vec![(6, 14, 4), (1, 8, 6), (1, 13, 11), (7, 12, 2), (4, 12, 3)].into_iter()
        {
            let (d0, _, Add(min)) = next.find(l, Add(0), |i, _| r < i);
            let (d1, _, Add(max)) = prev.find(r, Add(0), |i, _| i < l);
            assert_eq!(d0, d1);
            let ans = d0 + max - min + 1;
            assert_eq!(expected, ans);
        }
    }
}
