use bit_vec::BitVec;
use rand::Rng;

use crate::function::Function;

pub mod mu_plus_one;

pub trait Algorithm {
    fn run(&self, function: Box<dyn Function>) -> usize;
}
pub trait EvolutionaryAlgorithm: Algorithm {
    fn mutation_rate(&self) -> f64;

    fn mutate(&self, bitvec: &BitVec) -> BitVec {
        let mut rng = rand::thread_rng();
        let mutation_rate = self.mutation_rate();

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
}

pub trait GeneticAlgorithm: EvolutionaryAlgorithm {
    fn crossover_bias(&self) -> f64;

    fn crossover(&self, left: &BitVec, right: &BitVec) -> BitVec {
        let mut rng = rand::thread_rng();
        let crossover_bias = self.crossover_bias();

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
}
