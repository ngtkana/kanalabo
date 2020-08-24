// dbg {{{
#[allow(dead_code)]
mod dbg {
    #[macro_export]
    macro_rules! lg {
        () => {
            $crate::eprintln!("[{}:{}]", $crate::file!(), $crate::line!());
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
                $crate::eprintln!("[{}:{}][{}]", $crate::file!(), $crate::line!(), $msg);
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
use segtree_value::Value;
use std::{iter::successors, ops::Range};

#[derive(Debug, Clone)]
pub struct TwoDimensionalSegtree<T>
where
    T: Value,
{
    height: usize,
    width: usize,
    table: Vec<Vec<T>>,
}

impl<T> TwoDimensionalSegtree<T>
where
    T: Value,
{
    pub fn from_slice_vec(src: &[Vec<T>]) -> Self {
        let height = src.len();
        let width = src.get(0).map(|v| v.len()).unwrap_or(0);
        assert!(
            src.iter().all(|v| v.len() == width),
            "ジャギー配列は禁止です！"
        );
        let mut res = Self {
            table: src
                .iter()
                .map(|v| v.iter().cloned().cycle().take(2 * width).collect())
                .cycle()
                .take(2 * height)
                .collect(),
            height,
            width,
        };
        res.build();
        res
    }

    pub fn update(&mut self, i: usize, j: usize, x: T) {
        let orig_i = i + self.height;
        let orig_j = j + self.width;
        self.table[orig_i][orig_j] = x;
        for j in successors(Some(orig_j / 2), |x| Some(x / 2)).take_while(|&x| x != 0) {
            self.update_cell_horizontally_unckeckd(orig_i, j);
        }
        for i in successors(Some(orig_i / 2), |x| Some(x / 2)).take_while(|&x| x != 0) {
            for j in successors(Some(orig_j), |x| Some(x / 2)).take_while(|&x| x != 0) {
                self.update_cell_vertically_unckecked(i, j);
            }
        }
    }

    pub fn fold_horizontally(
        &self,
        i: usize,
        Range { mut start, mut end }: Range<usize>,
    ) -> Option<T> {
        assert!(start <= end, "変な区間を渡すのをやめませんか？");
        lg!((i, start, end));
        start += self.width;
        end += self.width;
        if start == end {
            None
        } else if start + 1 == end {
            Some(self.table[i][start].clone())
        } else {
            let row = &self.table[i];
            let mut left = row[start].clone();
            start += 1;
            end -= 1;
            let mut right = row[end].clone();
            while start != end {
                if start % 2 == 1 {
                    left.op_assign_from_the_right(&row[start]);
                    start += 1;
                }
                if end % 2 == 1 {
                    end -= 1;
                    right.op_assign_from_the_left(&row[end]);
                }
                start /= 2;
                end /= 2;
            }
            Some(left.op(&right))
        }
    }

    pub fn fold(
        &self,
        Range { mut start, mut end }: Range<usize>,
        range: Range<usize>,
    ) -> Option<T> {
        assert!(start <= end, "変な区間を渡すのをやめませんか？");
        assert!(range.start <= range.end, "変な区間を渡すのをやめませんか？");
        start += self.height;
        end += self.height;
        if start == end || range.start == range.end {
            None
        } else if start + 1 == end {
            self.fold_horizontally(start, range)
        } else {
            let mut left = self.fold_horizontally(start, range.clone()).unwrap();
            start += 1;
            end -= 1;
            let mut right = self.fold_horizontally(end, range.clone()).unwrap();
            while start != end {
                if start % 2 == 1 {
                    left.op_assign_from_the_right(
                        &self.fold_horizontally(start, range.clone()).unwrap(),
                    );
                    start += 1;
                }
                if end % 2 == 1 {
                    end -= 1;
                    right.op_assign_from_the_left(
                        &self.fold_horizontally(end, range.clone()).unwrap(),
                    );
                }
            }
            Some(left.op(&right))
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.table[self.height..]
            .iter()
            .map(|row| row[self.width..].iter().cloned().collect())
            .collect()
    }

    fn build(&mut self) {
        for i in self.height..self.height * 2 {
            for j in (1..self.width).rev() {
                self.update_cell_horizontally_unckeckd(i, j);
            }
        }
        for i in (1..self.height).rev() {
            for j in 1..self.width * 2 {
                self.update_cell_vertically_unckecked(i, j);
            }
        }
        tabular!(&self.table);
    }

    fn update_cell_horizontally_unckeckd(&mut self, i: usize, j: usize) {
        let (left, right) = self.table[i].split_at_mut(2 * j);
        left[j] = right[0].op(&right[1]);
    }

    fn update_cell_vertically_unckecked(&mut self, i: usize, j: usize) {
        let (upper, lower) = self.table.split_at_mut(2 * i);
        upper[i][j] = lower[0][j].op(&lower[1][j]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand() {
        use itertools::Itertools;
        use std::iter::once;

        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Cat(String);
        impl Value for Cat {
            fn op(&self, rhs: &Self) -> Self {
                Cat(self.0.chars().chain(rhs.0.chars()).collect())
            }
        }

        let table = ('a'..)
            .map(|c| Cat(once(c).collect()))
            .chunks(5)
            .into_iter()
            .map(|v| v.collect())
            .take(4)
            .collect::<Vec<_>>();

        let seg = TwoDimensionalSegtree::from_slice_vec(table.as_slice());

        assert_eq!(seg.fold(1..2, 2..3), Some(Cat("h".to_owned())));
        assert_eq!(seg.fold(1..3, 2..4), Some(Cat("himn".to_owned())));
        assert_eq!(
            seg.fold(0..4, 0..5),
            Some(Cat("abcdefghijklmnopqrst".to_owned()))
        );
        assert_eq!(seg.fold(1..4, 1..2), Some(Cat("glq".to_owned())));

        assert_eq!(
            seg.to_vec(),
            table
                .iter()
                .map(|row| row.iter().cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );
    }
}
