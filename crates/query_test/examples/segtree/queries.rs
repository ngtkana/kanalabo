use query_test::Query;
use std::{marker::PhantomData, ops::Range};

pub struct Set<T>(PhantomData<T>);
impl<T> Query for Set<T> {
    type Param = (usize, T);
    type Output = ();
    const NAME: &'static str = "set";
}

pub struct Fold<T>(PhantomData<T>);
impl<T> Query for Fold<T> {
    type Param = Range<usize>;
    type Output = T;
    const NAME: &'static str = "fold";
}
