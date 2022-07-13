use super::mu_plus_one;
use crate::{algorithm::Algorithm, function::Function};
use bit_vec::BitVec;
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
    fn run(&self, function: &impl Function) -> Vec<Vec<BitVec>> {
        mu_plus_one(
            self.mu,
            self.crossover_probability,
            self.mutation_rate,
            function,
            |_n, population| {
                population.pop();
            },
        )
    }
}
