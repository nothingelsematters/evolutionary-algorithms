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

impl Function for OneMax {
    fn n(&self) -> usize {
        self.n
    }

    fn fitness(&self, bitvec: &BitVec) -> i64 {
        ones(bitvec) as i64
    }

    fn is_local_optimum(&self, _bitvec: &BitVec) -> bool {
        false
    }

    fn is_best(&self, bitvec: &BitVec) -> bool {
        bitvec.all()
    }
}

impl Display for OneMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OneMax(n = {})", self.n)
    }
}
