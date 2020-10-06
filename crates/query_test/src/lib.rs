mod logger;

use rand::prelude::*;
use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    marker::PhantomData,
    ops::DerefMut,
};

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
    pub trait Judge<Q: Query> {
        fn judge(&self, param: Q::Param, output: Q::Output) -> bool;
    }
}

#[allow(dead_code)]
enum Config {
    Short,
    Verbose,
}

const CONFIG: Config = Config::Short;

#[derive(Debug, Clone, PartialEq)]
pub struct Tester<R, B, F, G> {
    rng: RefCell<R>,
    brute: B,
    fast: F,
    marker: PhantomData<G>,
}

impl<R, B, F, G> Tester<R, B, F, G>
where
    R: Rng,
    B: Init<G> + Debug + Clone,
    F: FromBrute<Brute = B> + Debug + Clone,
{
    pub fn rng_mut(&self) -> RefMut<R> {
        self.rng.borrow_mut()
    }
    pub fn new(mut rng: R) -> Self {
        let brute = B::init(&mut rng);
        let fast = F::from_brute(&brute);
        Self {
            rng: RefCell::new(rng),
            brute,
            fast,
            marker: PhantomData::<G>,
        }
    }
    pub fn initialize(&mut self) {
        let brute = B::init(self.rng_mut().deref_mut());
        let fast = F::from_brute(&brute);
        self.brute = brute;
        self.fast = fast;
    }
    pub fn compare<Q: Query>(&self)
    where
        Q::Param: Clone + Debug + Clone,
        Q::Output: Clone + Debug + Clone + PartialEq,
        B: Gen<Q, G> + solve::Solve<Q>,
        F: solve::Solve<Q>,
    {
        let param = self.brute.gen(self.rng_mut().deref_mut());
        let expected = self.brute.solve(param.clone());
        let output = self.fast.solve(param.clone());

        let verdict = expected == output;
        let logger = logger::Logger {
            tester: self,
            param,
            output: Some(output),
            expected: Some(expected),
            marker: PhantomData::<Q>,
        };
        match verdict {
            true => logger.passing(),
            false => {
                logger.failing();
                panic!("Failed in a test.");
            }
        }
    }
    pub fn compare_mut<Q: Query>(&mut self)
    where
        Q::Param: Clone + Debug + Clone,
        Q::Output: Clone + Debug + Clone + PartialEq,
        B: Gen<Q, G> + solve::SolveMut<Q>,
        F: solve::SolveMut<Q>,
    {
        let param = self.brute.gen(self.rng_mut().deref_mut());
        let expected = self.brute.solve_mut(param.clone());
        let output = self.fast.solve_mut(param.clone());

        let verdict = expected == output;
        let logger = logger::Logger {
            tester: self,
            param,
            output: Some(output),
            expected: Some(expected),
            marker: PhantomData::<Q>,
        };
        match verdict {
            true => logger.passing(),
            false => {
                logger.failing();
                panic!("Failed in a test.");
            }
        }
    }
    pub fn judge<Q: Query>(&self)
    where
        Q::Param: Clone + Debug,
        Q::Output: Clone + Debug + PartialEq,
        B: Gen<Q, G> + solve::Judge<Q>,
        F: solve::Solve<Q>,
    {
        let param = self.brute.gen(self.rng_mut().deref_mut());
        let output = self.fast.solve(param.clone());
        let verdict = self.brute.judge(param.clone(), output.clone());
        let logger = logger::Logger {
            tester: self,
            param,
            output: Some(output),
            expected: None,
            marker: PhantomData::<Q>,
        };
        match verdict {
            true => logger.passing(),
            false => {
                logger.failing();
                panic!("Failed in a test.");
            }
        }
    }
    pub fn mutate<Q: Query<Output = ()>>(&mut self)
    where
        Q::Param: Clone + Debug,
        Q::Output: Clone + Debug + PartialEq,
        B: Gen<Q, G> + solve::Mutate<Q>,
        F: solve::Mutate<Q>,
    {
        let param = self.brute.gen(self.rng_mut().deref_mut());
        self.brute.mutate(param.clone());
        self.fast.mutate(param.clone());
        logger::Logger {
            tester: self,
            param,
            output: None,
            expected: None,
            marker: PhantomData::<Q>,
        }
        .mutate();
    }
}
