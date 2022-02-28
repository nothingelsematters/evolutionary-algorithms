use super::{MuPlusOne, Mutant};
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
pub struct Common {
    mu: usize,
    crossover_probability: f64,
    mutation_rate: f64,
}

impl Common {
    pub fn new(mu: usize, crossover_probability: f64, mutation_rate: f64) -> Common {
        Common {
            mu,
            crossover_probability,
            mutation_rate,
        }
    }
}

impl MuPlusOne for Common {
    fn mu(&self) -> usize {
        self.mu
    }

    fn crossover_probability(&self) -> f64 {
        self.crossover_probability
    }

    fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    fn break_ties(&self, _n: usize, population: &mut BinaryHeap<Mutant>) {
        population.pop();
    }
}
