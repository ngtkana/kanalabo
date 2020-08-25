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
    pub rank: Vec<usize>,
    pub select: Vec<usize>,
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
}

#[derive(Debug, Clone)]
pub struct WaveletMatrix {
    height: usize,
    width: usize,
    table: Vec<WaveletMatrixRow>,
}

pub struct DebugWavletMatrix<'a>(&'a WaveletMatrix);

impl WaveletMatrix {
    pub fn from_vec_of_u32(mut src: Vec<u32>) -> Self {
        let height = 3;
        let table = (0..height)
            .rev()
            .map(|i| {
                let row = src.iter().map(|x| x >> i & 1 == 0).collect::<Vec<_>>();
                let (mut left, right) = src
                    .iter()
                    .copied()
                    .partition::<Vec<_>, _>(|x| x >> i & 1 == 0);
                std::mem::swap(&mut src, &mut left);
                src.extend(right);
                WaveletMatrixRow::from_vec_of_bool(row)
            })
            .collect::<Vec<_>>();

        Self {
            height,
            width: src.len(),
            table,
        }
    }
}

impl<'a> Debug for DebugWavletMatrix<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.0.table {
            writeln!(
                f,
                "{}",
                row.content
                    .iter()
                    .map(|&b| if b { '0' } else { '1' })
                    .collect::<String>()
            )?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand() {
        let a = [5, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0];
        let wavelet_matrix = WaveletMatrix::from_vec_of_u32(a.to_vec());
        println!("wavelet_matrix:\n{:?}", DebugWavletMatrix(&wavelet_matrix));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
