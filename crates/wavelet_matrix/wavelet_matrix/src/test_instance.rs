use super::{DebugWavletMatrixWithTab, WaveletMatrix};

use std::{
    fmt::Debug,
    ops::{Drop, Range},
};

pub(super) const VEC_SIZE: usize = 40;
pub(super) const VALUE_LIMIT_SMALL: u32 = 6;
pub(super) const VALUE_LIMIT_LARGE: u32 = 256;

fn random_value(value_limit: u32) -> u32 {
    rand::random::<u32>() % value_limit
}

fn random_vec(len: usize, value_limit: u32) -> Vec<u32> {
    std::iter::repeat_with(|| random_value(value_limit))
        .take(len)
        .collect()
}

#[derive(Debug, Clone)]
pub(super) struct TestInstance {
    pub value_limit: u32,
    pub vector: Vec<u32>,
    pub matrix: WaveletMatrix,
}

impl Drop for TestInstance {
    fn drop(&mut self) {
        self.info_end();
    }
}

#[derive(Debug, Clone)]
pub(super) struct IterationSpec {
    pub large_instance: usize,
    pub small_instance: usize,
    pub large_query: usize,
    pub small_query: usize,
}

impl TestInstance {
    pub fn new_large() -> Self {
        Self::with_len_and_value_max(VEC_SIZE, VALUE_LIMIT_LARGE)
    }

    pub fn new_small() -> Self {
        Self::with_len_and_value_max(VEC_SIZE, VALUE_LIMIT_SMALL)
    }

    fn with_len_and_value_max(len: usize, value_limit: u32) -> Self {
        let vector = random_vec(len, value_limit);
        let matrix = WaveletMatrix::from_vec_of_u32(vector.clone());
        let res = Self {
            value_limit,
            vector,
            matrix,
        };
        res.info_start();
        res
    }

    pub fn create_and_compare_many<T, I, A, F, G>(spec: &IterationSpec, init: I, f: F, g: G)
    where
        T: Debug + Eq,
        A: Debug,
        I: Fn(&Self) -> A,
        F: Fn(&Vec<u32>, &A) -> T,
        G: Fn(&WaveletMatrix, &A) -> T,
    {
        for _ in 0..spec.large_instance {
            let instance = Self::new_large();
            instance.compare_many(spec.large_query, &init, &f, &g);
        }
        for _ in 0..spec.small_instance {
            let instance = Self::new_small();
            instance.compare_many(spec.small_query, &init, &f, &g);
        }
    }

    pub fn compare_many<T, I, A, F, G>(&self, iter: usize, init: &I, f: &F, g: &G)
    where
        T: Debug + Eq,
        A: Debug,
        I: Fn(&Self) -> A,
        F: Fn(&Vec<u32>, &A) -> T,
        G: Fn(&WaveletMatrix, &A) -> T,
    {
        println!("Queries and Results:");
        println!("\tQuery\tExpect\tResult");
        std::iter::repeat_with(|| init(&self))
            .take(iter)
            .for_each(|x| {
                let expected = f(&self.vector, &x);
                let result = g(&self.matrix, &x);
                println!("\t{:?}\t{:?}\t{:?}", x, expected, result);
            });
    }

    pub fn random_index(&self) -> usize {
        rand::random::<usize>() % self.vector.len()
    }

    pub fn random_range(&self) -> Range<usize> {
        let mut start = self.random_index();
        let mut end = self.random_index();
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        end += 1;
        Range { start, end }
    }

    pub fn random_value(&self) -> u32 {
        rand::random::<u32>() % self.value_limit
    }

    pub fn count(&self, x: u32) -> usize {
        self.vector.iter().filter(|&&y| y == x).count()
    }

    fn info_start(&self) {
        println!("Created an instance.\n");
        println!("vector:\n\t{:?}", &self.vector);
        println!("matrix:\n{:?}", DebugWavletMatrixWithTab(&self.matrix));
        println!();
    }

    fn info_end(&self) {
        println!("Dropped an instance.");
        println!();
        println!();
    }
}
