use crate::{
    algorithm::{initialize_random, Algorithm, Mutant},
    function::Function,
};
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
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

impl Display for Common {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(μ + 1), μ = {}, p_c = {}, p_m = {}",
            self.mu, self.crossover_probability, self.mutation_rate,
        )
    }
}

impl Algorithm for Common {
    fn initialize<F: Function>(&self, function: &F) -> Vec<Mutant> {
        initialize_random(self.mu, function)
    }

    fn iterate<F: Function>(&self, population: &mut Vec<Mutant>, function: &F) {
        super::mu_plus_one_iterate(
            self.crossover_probability,
            self.mutation_rate,
            function,
            population,
            |_n, population| {
                let index = population
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, x)| x.fitness)
                    .unwrap()
                    .0;
                population.remove(index);
            },
        )
    }
}
