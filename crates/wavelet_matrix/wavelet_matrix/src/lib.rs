// dbg {{{
#[allow(dead_code)]
mod dbg {
    #[macro_export]
    macro_rules! lg {
        () => {
            eprintln!("[{}:{}]", file!(), line!());
        };
        ($val:expr) => {
            match $val {
                tmp => {
                    eprintln!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($val), &tmp);
                    tmp
                }
            }
        };
        ($val:expr,) => { lg!($val) };
        ($($val:expr),+ $(,)?) => {
            ($(lg!($val)),+,)
        };
    }

    #[macro_export]
    macro_rules! msg {
            () => {
                compile_error!();
            };
            ($msg:expr) => {
                eprintln!("[{}:{}][{}]", file!(), line!(), $msg);
            };
            ($msg:expr, $val:expr) => {
                match $val {
                    tmp => {
                        eprintln!("[{}:{}][{}] {} = {:?}",
                            file!(), line!(), $msg, stringify!($val), &tmp);
                        tmp
                    }
                }
            };
            ($msg:expr, $val:expr,) => { msg!($msg, $val) };
            ($msg:expr, $($val:expr),+ $(,)?) => {
                ($(msg!($msg, $val)),+,)
            };
        }

    #[macro_export]
    macro_rules! tabular {
        ($val:expr) => {
            eprintln!(
                "[{}:{}] {}:\n{:?}",
                file!(),
                line!(),
                stringify!($val),
                crate::dbg::Tabular($val)
            );
        };
    }

    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [T]);
    impl<'a, T: Debug> Debug for Tabular<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, &self.0[i])?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanTable<'a>(pub &'a [Vec<bool>]);
    impl<'a> Debug for BooleanTable<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, BooleanSlice(&self.0[i]))?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanSlice<'a>(pub &'a [bool]);
    impl<'a> Debug for BooleanSlice<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|&b| if b { "1 " } else { "0 " })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}

use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub struct WaveletMatrixRow {
    pub content: Vec<bool>,
    pub rank: Vec<usize>,   // [0, i[ 内の 1 の数です。
    pub select: Vec<usize>, // rank.lower_bound(i) です。
}

impl WaveletMatrixRow {
    pub fn from_vec_of_bool(src: Vec<bool>) -> Self {
        let mut rank = vec![0; src.len() + 1];
        let mut select = vec![0];
        for (i, &b) in src.iter().enumerate() {
            rank[i + 1] = if b { rank[i] + 1 } else { rank[i] };
            if b {
                select.push(i + 1);
            }
        }
        Self {
            content: src,
            rank,
            select,
        }
    }

    // j -> (content[j], j) をソート順に並べたときの、
    // lower_bound((b, i)) です。
    pub fn enumerate_lower_bound(&self, b: bool, i: usize) -> usize {
        if b {
            self.content.len() - self.rank[self.content.len()] + self.rank[i]
        } else {
            i - self.rank[i]
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
