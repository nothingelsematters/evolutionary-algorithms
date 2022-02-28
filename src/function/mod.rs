use bit_vec::BitVec;

pub mod jump;

pub trait Function {
    fn fitness(&self, bitvec: &BitVec) -> i64;

    fn n(&self) -> usize;

    fn is_best(&self) -> BitVec;

    fn is_local_optimum(&self, bitvec: &BitVec) -> bool;
}
