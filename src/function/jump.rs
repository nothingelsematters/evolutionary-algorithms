use super::Function;
use crate::utils::ones;
use bit_vec::BitVec;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Jump {
    n: usize,
    k: usize,
}

impl Jump {
    pub fn new(n: usize, k: usize) -> Jump {
        Jump { n, k }
    }
}

impl Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jump(n = {}, k = {})", self.n, self.k)
    }
}

impl Function for Jump {
    fn n(&self) -> usize {
        self.n
    }

    fn fitness(&self, bitvec: &BitVec) -> i64 {
        let ones = ones(bitvec);

        if (ones <= self.n - self.k) || (ones == self.n) {
            ones as i64
        } else {
            self.n as i64 - ones as i64 - self.k as i64
        }
    }

    fn best_fitness(&self) -> i64 {
        self.n as i64
    }
}
