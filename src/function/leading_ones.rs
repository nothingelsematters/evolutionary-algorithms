use super::Function;
use bit_vec::BitVec;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct LeadingOnes {
    n: usize,
}

impl LeadingOnes {
    pub fn new(n: usize) -> LeadingOnes {
        LeadingOnes { n }
    }
}

impl Display for LeadingOnes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LeadingOnes(n = {})", self.n)
    }
}

impl Function for LeadingOnes {
    fn n(&self) -> usize {
        self.n
    }

    fn fitness(&self, bitvec: &BitVec) -> i64 {
        if self.n <= u32::BITS as usize {
            bitvec.storage()[0].leading_ones() as i64
        } else {
            bitvec
                .iter()
                .position(|x| !x)
                .unwrap_or_else(|| bitvec.len()) as i64
        }
    }

    fn best_fitness(&self) -> i64 {
        self.n as i64
    }
}
