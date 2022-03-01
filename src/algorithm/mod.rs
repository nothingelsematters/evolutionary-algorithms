use crate::function::Function;
use bit_vec::BitVec;
use lazy_static::lazy_static;
use rand::Rng;
use std::{
    cmp::Ordering,
    sync::atomic::{AtomicUsize, Ordering as AtomicOrdering},
};

pub mod mu_plus_one;

pub trait Algorithm {
    fn run(&self, function: impl Function);
}

fn mutate(bitvec: &BitVec, mutation_rate: f64) -> BitVec {
    let mut rng = rand::thread_rng();

    bitvec
        .iter()
        .map(|x| {
            if rng.gen::<f64>() < mutation_rate {
                !x
            } else {
                x
            }
        })
        .collect()
}

fn crossover(left: &BitVec, right: &BitVec, crossover_bias: f64) -> BitVec {
    let mut rng = rand::thread_rng();

    left.iter()
        .zip(right)
        .map(|(x, y)| {
            if rng.gen::<f64>() > crossover_bias {
                x
            } else {
                y
            }
        })
        .collect()
}

// a crutch for multi item heap
lazy_static! {
    static ref MUTANT_NUMBER: AtomicUsize = AtomicUsize::new(0);
}

#[derive(PartialEq, Eq)]
struct Mutant {
    fitness: i64,
    bitvec: BitVec,
    crutch: usize,
}

impl PartialOrd for Mutant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // a crutch for a min heap
        let result = match self.fitness.cmp(&other.fitness) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => (&self.bitvec, self.crutch).cmp(&(&other.bitvec, other.crutch)),
        };
        Some(result)
    }
}

impl Ord for Mutant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Mutant {
    fn new(bitvec: BitVec, function: &dyn Function) -> Mutant {
        Mutant {
            fitness: function.fitness(&bitvec),
            bitvec,
            crutch: MUTANT_NUMBER.fetch_add(1, AtomicOrdering::SeqCst),
        }
    }
}
