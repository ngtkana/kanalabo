use std::fmt::{Debug, Formatter};

#[cfg(test)]
mod test_instance;

#[derive(Debug, Clone)]
pub struct WaveletMatrixRow {
    pub content: Vec<bool>,
    pub rank: Vec<usize>,   // [0, i[ 内の 1 の数です。
    pub zeroes: Vec<usize>, // 0 の登場箇所です。
    pub ones: Vec<usize>,   // 1 の登場箇所です。
}

impl WaveletMatrixRow {
    pub fn from_vec_of_bool(src: Vec<bool>) -> Self {
        let mut rank = vec![0; src.len() + 1];
        let mut zeroes = Vec::new();
        let mut ones = Vec::new();
        for (i, &b) in src.iter().enumerate() {
            rank[i + 1] = if b { rank[i] + 1 } else { rank[i] };
            if b {
                ones.push(i);
            } else {
                zeroes.push(i);
            }
        }
        Self {
            content: src,
            rank,
            zeroes,
            ones,
        }
    }

    // j -> (content[j], j) をソート順に並べたときの、
    // lower_bound((b, i)) です。
    pub fn enumerate_lower_bound(&self, b: bool, i: usize) -> usize {
        if b {
            self.zeroes.len() + self.rank[i]
        } else {
            i - self.rank[i]
        }
    }

    // j -> (content[j], j) をソート順に並べたときの i 番目です。
    pub fn enumerate_sorted(&self, i: usize) -> (bool, usize) {
        if i < self.zeroes.len() {
            (false, self.zeroes[i])
        } else {
            (true, self.ones[i - self.zeroes.len()])
        }
    }
}

#[derive(Debug, Clone)]
pub struct WaveletMatrix {
    height: usize,
    width: usize,
    table: Vec<WaveletMatrixRow>,
}

impl WaveletMatrix {
    pub fn from_vec_of_u32(mut src: Vec<u32>) -> Self {
        let height = src
            .iter()
            .max()
            .unwrap()
            .next_power_of_two()
            .trailing_zeros() as usize;
        let table = (0..height)
            .rev()
            .map(|i| {
                let row = WaveletMatrixRow::from_vec_of_bool(
                    src.iter().map(|x| x >> i & 1 == 1).collect::<Vec<_>>(),
                );
                let (mut left, right) = src
                    .iter()
                    .copied()
                    .partition::<Vec<_>, _>(|x| x >> i & 1 == 0);
                std::mem::swap(&mut src, &mut left);
                src.extend(right);
                row
            })
            .collect::<Vec<_>>();

        Self {
            height,
            width: src.len(),
            table,
        }
    }

    pub fn access(&self, mut i: usize) -> u32 {
        let mut ans = 0;
        for row in &self.table {
            ans *= 2;
            if row.content[i] {
                ans += 1;
            }
            i = row.enumerate_lower_bound(row.content[i], i);
        }
        ans
    }

    pub fn rank(&self, x: u32, mut i: usize) -> usize {
        assert!(x < 1 << self.height, "入力の値が 2 の高さ乗以上です。");
        let mut l = 0;
        for (row, b) in self
            .table
            .iter()
            .zip(Self::bits_from_the_top(self.height, x))
        {
            l = row.enumerate_lower_bound(b, l);
            i = row.enumerate_lower_bound(b, i);
        }
        i - l
    }

    pub fn select(&self, x: u32, i: usize) -> usize {
        assert!(x < 1 << self.height, "入力の値が 2 の高さ乗以上です。");
        let l = self.go_down(0, x);
        let r = self.go_down(self.width, x);
        assert!(
            l + i < r,
            "{} は {} 個しかありませんが、 select({}, {}) が呼ばれました。",
            x,
            r - l,
            x,
            i
        );
        let mut i = l + i;
        for row in self.table.iter().rev() {
            i = row.enumerate_sorted(i).1;
        }
        i
    }

    // 一段目で i だったものが x のビットに従って降りた結果です。
    fn go_down(&self, mut i: usize, x: u32) -> usize {
        assert!(x < 1 << self.height, "入力の値が 2 の高さ乗以上です。");
        for (row, b) in self
            .table
            .iter()
            .zip(Self::bits_from_the_top(self.height, x))
        {
            i = row.enumerate_lower_bound(b, i);
        }
        i
    }

    fn bits_from_the_top(height: usize, x: u32) -> Vec<bool> {
        (0..height).rev().map(|i| x >> i & 1 == 1).collect()
    }
}

pub struct DebugWavletMatrixRow<'a>(&'a WaveletMatrixRow);
impl<'a> Debug for DebugWavletMatrixRow<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .content
                .iter()
                .map(|&b| if b { '1' } else { '0' })
                .collect::<String>()
        )
    }
}

pub struct DebugWavletMatrix<'a>(&'a WaveletMatrix);
impl<'a> Debug for DebugWavletMatrix<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.0.table {
            writeln!(f, "{:?}", DebugWavletMatrixRow(&row))?
        }
        Ok(())
    }
}

pub struct DebugWavletMatrixWithTab<'a>(&'a WaveletMatrix);
impl<'a> Debug for DebugWavletMatrixWithTab<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.0.table {
            writeln!(f, "\t{:?}", DebugWavletMatrixRow(&row))?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_instance::*;

    fn sample_hand() -> WaveletMatrix {
        let a = [5, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0];
        WaveletMatrix::from_vec_of_u32(a.to_vec())
    }

    #[test]
    fn test_hand_access() {
        let a = sample_hand();
        print!("a\n{:?}", DebugWavletMatrix(&a));
        assert_eq!(a.access(0), 5);
        assert_eq!(a.access(1), 4);
        assert_eq!(a.access(2), 5);
        assert_eq!(a.access(3), 5);
        assert_eq!(a.access(4), 2);
        assert_eq!(a.access(5), 1);
        assert_eq!(a.access(6), 5);
        assert_eq!(a.access(7), 6);
        assert_eq!(a.access(8), 1);
        assert_eq!(a.access(9), 3);
        assert_eq!(a.access(10), 5);
        assert_eq!(a.access(11), 0);
    }

    #[test]
    fn test_hand_rank() {
        let a = sample_hand();
        print!("a\n{:?}", DebugWavletMatrix(&a));

        assert_eq!(a.rank(5, 0), 0);
        assert_eq!(a.rank(5, 1), 1);
        assert_eq!(a.rank(5, 2), 1);
        assert_eq!(a.rank(5, 3), 2);
        assert_eq!(a.rank(5, 4), 3);
        assert_eq!(a.rank(5, 5), 3);
        assert_eq!(a.rank(5, 6), 3);
        assert_eq!(a.rank(5, 7), 4);
        assert_eq!(a.rank(5, 8), 4);
        assert_eq!(a.rank(5, 9), 4);
        assert_eq!(a.rank(5, 10), 4);
        assert_eq!(a.rank(5, 11), 5);
        assert_eq!(a.rank(5, 12), 5);
    }

    #[test]
    fn test_hand_select() {
        let a = sample_hand();
        print!("a\n{:?}", DebugWavletMatrix(&a));

        assert_eq!(a.select(5, 0), 0);
        assert_eq!(a.select(5, 1), 2);
        assert_eq!(a.select(5, 2), 3);
        assert_eq!(a.select(5, 3), 6);
    }

    const ITERATION: usize = 3;

    #[test]
    fn test_random_access_large() {
        for _ in 0..ITERATION {
            let instance = TestInstance::new_large();
            instance.compare_many(
                10,
                |me| me.random_index(),
                |vector, &i| vector[i],
                |matrix, &i| matrix.access(i),
            );
        }
    }

    #[test]
    fn test_random_access_small() {
        for _ in 0..ITERATION {
            let instance = TestInstance::new_small();
            instance.compare_many(
                10,
                |me| me.random_index(),
                |vector, &i| vector[i],
                |matrix, &i| matrix.access(i),
            );
        }
    }

    #[test]
    fn test_random_rank_large() {
        for _ in 0..ITERATION {
            let instance = TestInstance::new_large();
            instance.compare_many(
                10,
                |me| (me.random_value(), me.random_index()),
                |vector, &(x, i)| vector[i..].iter().filter(|&&y| y == x).count(),
                |matrix, &(x, i)| matrix.rank(x, i),
            );
        }
    }

    #[test]
    fn test_random_rank_small() {
        for _ in 0..ITERATION {
            let instance = TestInstance::new_small();
            instance.compare_many(
                10,
                |me| (me.random_value(), me.random_index()),
                |vector, &(x, i)| vector[i..].iter().filter(|&&y| y == x).count(),
                |matrix, &(x, i)| matrix.rank(x, i),
            );
        }
    }

    #[test]
    fn test_random_select_large() {
        for _ in 0..ITERATION {
            let instance = TestInstance::new_large();
            instance.compare_many(
                10,
                |me| {
                    let x = me.vector[me.random_index()];
                    let i = rand::random::<usize>() % me.count(x);
                    (x, i)
                },
                |vector, &(x, i)| {
                    vector
                        .iter()
                        .enumerate()
                        .filter(|&(_, &y)| y == x)
                        .nth(i)
                        .unwrap()
                        .0
                },
                |matrix, &(x, i)| matrix.select(x, i),
            );
        }
    }

    #[test]
    fn test_random_select_small() {
        for _ in 0..ITERATION {
            let instance = TestInstance::new_small();
            instance.compare_many(
                10,
                |me| {
                    let x = me.vector[me.random_index()];
                    let i = rand::random::<usize>() % me.count(x);
                    (x, i)
                },
                |vector, &(x, i)| {
                    vector
                        .iter()
                        .enumerate()
                        .filter(|&(_, &y)| y == x)
                        .nth(i)
                        .unwrap()
                        .0
                },
                |matrix, &(x, i)| matrix.select(x, i),
            );
        }
    }
}
