use segtree_value::Value;
use std::ops::Range;

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
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Cat(String);
        impl Value for Cat {
            fn op(&self, b: &Cat) -> Cat {
                Cat(self.0.chars().chain(b.0.chars()).collect())
            }
        }
        use std::iter::once;
        let a = ('0'..='9')
            .map(|c| Cat(once(c).collect::<String>()))
            .collect::<Vec<_>>();
        let mut seg = Segtree::with_slice(a.as_slice());

        assert_eq!(seg.fold(3..5), Some(Cat("34".to_owned())));
        assert_eq!(seg.fold(2..9), Some(Cat("2345678".to_owned())));
        assert_eq!(seg.fold(0..4), Some(Cat("0123".to_owned())));
        assert_eq!(seg.fold(8..8), None);

        seg.set(3, Cat("d".to_owned()));
        seg.set(6, Cat("g".to_owned()));
        assert_eq!(seg.fold(0..4), Some(Cat("012d".to_owned())));
        assert_eq!(seg.fold(2..9), Some(Cat("2d45g78".to_owned())));
    }
}
