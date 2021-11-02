use bit_vec::BitVec;

use super::Function;

pub struct ShiftedValleyJump {
    n: usize,
    k: usize,
    valley_center: BitVec,
}

impl ShiftedValleyJump {
    pub fn new(n: usize, k: usize, valley_center: BitVec) -> ShiftedValleyJump {
        ShiftedValleyJump {
            n,
            k,
            valley_center,
        }
    }
}

impl Function for ShiftedValleyJump {
    fn fitness(&self, bitvec: &BitVec) -> usize {
        let ones = bitvec
            .iter()
            .zip(&self.valley_center)
            .filter(|(x, y)| x == y)
            .count();

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
        self.valley_center.clone()
    }

    fn is_local_optimum(&self, bitvec: &BitVec) -> bool {
        bitvec.iter().filter(|x| *x).count() == self.n - self.k
    }
}
