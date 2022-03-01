use bit_vec::BitVec;

pub trait Function {
    fn n(&self) -> usize;

    fn fitness(&self, bitvec: &BitVec) -> i64;

    fn is_local_optimum(&self, bitvec: &BitVec) -> bool;

    fn is_best(&self, bitvec: &BitVec) -> bool;
}

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
    fn n(&self) -> usize {
        self.n
    }

    fn fitness(&self, bitvec: &BitVec) -> i64 {
        let ones = bitvec.iter().filter(|x| *x).count();

        if (ones <= self.n - self.k) || (ones == self.n) {
            ones as i64
        } else {
            self.n as i64 - ones as i64 - self.k as i64
        }
    }

    fn is_local_optimum(&self, bitvec: &BitVec) -> bool {
        bitvec.iter().filter(|x| *x).count() == self.n - self.k
    }

    fn is_best(&self, bitvec: &BitVec) -> bool {
        *bitvec == BitVec::from_elem(self.n, true)
    }
}
