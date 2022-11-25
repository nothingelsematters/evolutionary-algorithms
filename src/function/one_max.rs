use super::Function;
use crate::utils::ones;
use bit_vec::BitVec;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct OneMax {
    n: usize,
}

impl OneMax {
    pub fn new(n: usize) -> OneMax {
        OneMax { n }
    }
}

impl Display for OneMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OneMax(n = {})", self.n)
    }
}

impl Function for OneMax {
    fn n(&self) -> usize {
        self.n
    }

    fn fitness(&self, bitvec: &BitVec) -> i64 {
        ones(bitvec) as i64
    }

    fn best_fitness(&self) -> i64 {
        self.n as i64
    }
}
