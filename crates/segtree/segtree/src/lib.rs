use std::{fmt::Debug, ops::Range};

pub trait Value: Debug + Clone {
    fn op(&self, rhs: &Self) -> Self;

    fn op_assign_from_the_right(&mut self, rhs: &Self) {
        *self = self.op(rhs);
    }

    fn op_assign_from_the_left(&mut self, rhs: &Self) {
        *self = rhs.op(self);
    }
}

#[derive(Debug, Clone)]
pub struct Segtree<T>
where
    T: Value,
{
    len: usize,
    table: Vec<T>,
}

impl<T: Value> Segtree<T> {
    pub fn with_slice(src: &[T]) -> Self {
        let mut res = Self {
            table: src.iter().cloned().cycle().take(src.len() * 2).collect(),
            len: src.len(),
        };
        res.build();
        res
    }

    fn update_node_unckecked(&mut self, i: usize) {
        let (left, right) = self.table.split_at_mut(i * 2);
        left[i] = right[0].op(&right[1]);
    }

    fn build(&mut self) {
        (1..self.len)
            .rev()
            .for_each(|i| self.update_node_unckecked(i))
    }

    pub fn set(&mut self, mut i: usize, x: T) {
        i += self.len;
        self.table[i] = x;
        while i != 1 {
            i /= 2;
            self.update_node_unckecked(i);
        }
    }

    pub fn fold(&self, Range { mut start, mut end }: Range<usize>) -> Option<T> {
        assert!(start <= end, "変な区間を渡すのをやめませんか？");
        start += self.len;
        end += self.len;
        if start == end {
            None
        } else if start + 1 == end {
            Some(self.table[start].clone())
        } else {
            let mut left = self.table[start].clone();
            start += 1;
            end -= 1;
            let mut right = self.table[end].clone();
            while start != end {
                if start % 2 == 1 {
                    left.op_assign_from_the_right(&self.table[start]);
                    start += 1;
                }
                if end % 2 == 1 {
                    end -= 1;
                    right.op_assign_from_the_left(&self.table[end]);
                }
                start /= 2;
                end /= 2;
            }
            Some(left.op(&right))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_concatenation() {
        impl Value for String {
            fn op(&self, b: &String) -> String {
                self.chars().chain(b.chars()).collect()
            }
        }
        use std::iter::once;
        let a = ('0'..='9')
            .map(|c| once(c).collect::<String>())
            .collect::<Vec<_>>();
        let mut seg = Segtree::with_slice(a.as_slice());

        assert_eq!(seg.fold(3..5), Some("34".to_owned()));
        assert_eq!(seg.fold(2..9), Some("2345678".to_owned()));
        assert_eq!(seg.fold(0..4), Some("0123".to_owned()));
        assert_eq!(seg.fold(8..8), None);

        seg.set(3, "d".to_owned());
        seg.set(6, "g".to_owned());
        assert_eq!(seg.fold(0..4), Some("012d".to_owned()));
        assert_eq!(seg.fold(2..9), Some("2d45g78".to_owned()));
    }
}
