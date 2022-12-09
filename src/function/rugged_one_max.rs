use super::Function;
use crate::utils::ones;
use bit_vec::BitVec;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct RuggedOneMax {
    k: usize,
    n: usize,
}

impl RuggedOneMax {
    pub fn new(k: usize, n: usize) -> RuggedOneMax {
        RuggedOneMax { k, n }
    }
}

impl Display for RuggedOneMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuggedOneMax(k = {}, n = {})", self.k, self.n)
    }
}

impl Function for RuggedOneMax {
    fn n(&self) -> usize {
        self.n
    }

    fn fitness(&self, bitvec: &BitVec) -> i64 {
        let ones = ones(bitvec);
        let optimum_distance = self.n - ones;

        let result = match self.k {
            2 => match optimum_distance % 2 {
                0 => ones + 1,
                1 => ones - 1,
                _ => unreachable!(),
            },
            3 => match optimum_distance % 3 {
                0 => ones + 3,
                1 => ones + 1,
                2 => ones - 1,
                _ => unreachable!(),
            },
            _ => todo!(),
        };
        result as i64
    }

    fn best_fitness(&self) -> i64 {
        let additional = match self.k {
            2 => 1,
            3 => 3,
            _ => todo!(),
        };
        (self.n + additional) as i64
    }
}
