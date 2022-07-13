use bit_vec::BitVec;

pub fn ones(bitvec: &BitVec) -> usize {
    bitvec.iter().filter(|x| *x).count()
}
