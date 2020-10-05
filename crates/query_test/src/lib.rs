use rand::prelude::*;

pub trait Init<G> {
    fn init(rng: &mut impl Rng) -> Self;
}
pub trait FromBrute {
    type Brute;
    fn from_brute(brute: &Self::Brute) -> Self;
}
pub trait Query {
    type Param;
    type Output;
    const NAME: &'static str;
}
pub trait Gen<Q: Query, G> {
    fn gen(&self, rng: &mut impl Rng) -> Q::Param;
}

pub mod solve {
    use crate::Query;
    pub trait Solve<Q: Query> {
        fn solve(&self, param: Q::Param) -> Q::Output;
    }
    pub trait SolveMut<Q: Query> {
        fn solve_mut(&mut self, param: Q::Param) -> Q::Output;
    }
    pub trait Mutate<Q: Query<Output = ()>> {
        fn mutate(&mut self, param: Q::Param);
    }
}
