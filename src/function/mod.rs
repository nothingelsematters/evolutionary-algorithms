use bit_vec::BitVec;

mod jump;
mod leading_ones;
mod one_max;

pub use jump::Jump;
pub use leading_ones::LeadingOnes;
pub use one_max::OneMax;

pub trait Function {
    fn n(&self) -> usize;

    fn fitness(&self, bitvec: &BitVec) -> i64;

    fn is_local_optimum(&self, bitvec: &BitVec) -> bool;

    fn is_best(&self, bitvec: &BitVec) -> bool;
}
