use std::fmt::Debug;

pub trait Value: Debug + Clone {
    fn op(&self, rhs: &Self) -> Self;

    fn op_assign_from_the_right(&mut self, rhs: &Self) {
        *self = self.op(rhs);
    }

    fn op_assign_from_the_left(&mut self, rhs: &Self) {
        *self = rhs.op(self);
    }
}
