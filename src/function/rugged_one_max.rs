use super::Function;
use crate::utils::ones;
use bit_vec::BitVec;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct RuggedOneMax {
    n: usize,
}

impl RuggedOneMax {
    pub fn new(n: usize) -> RuggedOneMax {
        RuggedOneMax { n }
    }
}

impl Function for RuggedOneMax {
    fn n(&self) -> usize {
        self.n
    }

    fn fitness(&self, bitvec: &BitVec) -> i64 {
        let ones = ones(bitvec) as i64;
        if ones & 1 == 0 {
            ones + 1
        } else {
            ones - 1
        }
    }

    fn is_local_optimum(&self, _bitvec: &BitVec) -> bool {
        false
    }

    fn is_best(&self, bitvec: &BitVec) -> bool {
        bitvec.all()
    }
}

impl Display for RuggedOneMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuggedOneMax(n = {})", self.n)
    }
}