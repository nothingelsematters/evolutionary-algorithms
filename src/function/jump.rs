use bit_vec::BitVec;

use super::Function;

#[derive(Clone, Copy)]
pub struct Jump {
    n: usize,
    k: usize,
}

impl Jump {
    pub fn new(n: usize, k: usize) -> Jump {
        Jump { n, k }
    }
}

impl Function for Jump {
    fn fitness(&self, bitvec: &BitVec) -> usize {
        let ones = bitvec.iter().filter(|x| *x).count();

        if (ones <= self.n - self.k) || (ones == self.n) {
            ones + self.k
        } else {
            self.n - ones
        }
    }

    fn n(&self) -> usize {
        self.n
    }

    fn is_best(&self) -> BitVec {
        (0..self.n).map(|_| true).collect()
    }

    fn is_local_optimum(&self, bitvec: &BitVec) -> bool {
        bitvec.iter().filter(|x| *x).count() == self.n - self.k
    }
}
