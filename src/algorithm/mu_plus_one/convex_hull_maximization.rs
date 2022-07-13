use super::mu_plus_one;
use crate::{
    algorithm::{Algorithm, Mutant},
    function::Function,
};
use bit_vec::BitVec;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct ConvexHullMaximization {
    mu: usize,
    crossover_probability: f64,
    mutation_rate: f64,
}

impl Display for ConvexHullMaximization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(μ + 1) w/ chm, μ = {}, p_c = {}, p_m = {}",
            self.mu, self.crossover_probability, self.mutation_rate,
        )
    }
}

impl Algorithm for ConvexHullMaximization {
    fn run(&self, function: &impl Function) -> Vec<Vec<BitVec>> {
        mu_plus_one(
            self.mu,
            self.crossover_probability,
            self.mutation_rate,
            function,
            ConvexHullMaximization::break_ties,
        )
    }
}

impl ConvexHullMaximization {
    pub fn new(
        mu: usize,
        crossover_probability: f64,
        mutation_rate: f64,
    ) -> ConvexHullMaximization {
        ConvexHullMaximization {
            mu,
            crossover_probability,
            mutation_rate,
        }
    }

    fn different_ones(bitvec: &BitVec, current_different_ones: Vec<u8>) -> Vec<u8> {
        bitvec
            .iter()
            .zip(current_different_ones)
            .map(|(bit, value)| value | (if bit { 0b10 } else { 0b1 }))
            .collect()
    }

    fn break_ties(n: usize, population: &mut Vec<Mutant>) {
        let lowest_fitness = population.iter().map(|x| x.fitness).min().unwrap();
        let mut choosing = Vec::new();

        let mut i = 0;
        while i < population.len() {
            if population[i].fitness == lowest_fitness {
                choosing.push(population.remove(i));
            }
            i += 1;
        }

        let different_ones = population.iter().fold(vec![0u8; n], |init, m| {
            ConvexHullMaximization::different_ones(&m.bitvec, init)
        });

        let i_terminated = (0..choosing.len())
            .map(|i| {
                let mut different_ones = different_ones.clone();

                for choosing_j in choosing
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, x)| x)
                {
                    different_ones =
                        ConvexHullMaximization::different_ones(&choosing_j.bitvec, different_ones);
                }

                let value: usize = different_ones
                    .iter()
                    .map(|x| match x {
                        1 | 2 => 1,
                        3 => 2,
                        _ => panic!(),
                    })
                    .sum();
                (i, value)
            })
            .min_by(|(_, x_value), (_, y_value)| x_value.cmp(y_value))
            .unwrap()
            .0;

        for (i, mutant) in choosing.into_iter().enumerate() {
            if i != i_terminated {
                population.push(mutant);
            }
        }
    }
}
