use bit_vec::BitVec;

mod jump;
mod leading_ones;
mod one_max;
mod rugged_one_max;

pub use jump::Jump;
pub use leading_ones::LeadingOnes;
pub use one_max::OneMax;
pub use rugged_one_max::RuggedOneMax;

pub trait Function {
    fn n(&self) -> usize;

    fn fitness(&self, bitvec: &BitVec) -> i64;

    fn best_fitness(&self) -> i64;
}
