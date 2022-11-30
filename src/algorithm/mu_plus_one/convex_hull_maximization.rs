use crate::{
    algorithm::{initialize_random, Algorithm, Mutant},
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
    fn initialize<F: Function>(&self, function: &F) -> Vec<Mutant> {
        initialize_random(self.mu, function)
    }

    fn fitness_evaluations(&self, iterations: usize) -> usize {
        iterations + self.mu
    }

    fn iterate<F: Function>(&self, population: &mut Vec<Mutant>, function: &F) {
        super::mu_plus_one_iterate(
            self.crossover_probability,
            self.mutation_rate,
            function,
            population,
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

    fn different_ones(bitvec: &BitVec, current_different_ones: &mut [u8]) {
        current_different_ones
            .iter_mut()
            .zip(bitvec)
            .for_each(|(value, bit)| *value |= if bit { 0b10 } else { 0b01 })
    }

    fn break_ties(n: usize, population: &mut Vec<Mutant>) {
        let lowest_fitness = population.iter().map(|x| x.fitness).min().unwrap();
        let choosing: Vec<_> = population
            .iter()
            .enumerate()
            .filter(|(_, x)| x.fitness == lowest_fitness)
            .map(|(i, _)| i)
            .collect();

        let different_ones = population
            .iter()
            .filter(|x| x.fitness != lowest_fitness)
            .fold(vec![0u8; n], |mut init, m| {
                ConvexHullMaximization::different_ones(&m.bitvec, &mut init);
                init
            });

        let i_terminated = (0..choosing.len())
            .map(|i| {
                let mut different_ones = different_ones.clone();

                choosing
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .for_each(|(_, choosing_j)| {
                        ConvexHullMaximization::different_ones(
                            &population[*choosing_j].bitvec,
                            &mut different_ones,
                        );
                    });

                let value: usize = different_ones
                    .iter()
                    .map(|x| match x {
                        1 | 2 => 1,
                        3 => 2,
                        _ => panic!(),
                    })
                    .sum();

                (choosing[i], value)
            })
            .max_by_key(|(_, value)| *value)
            .map(|(i, _)| i)
            .unwrap();

        population.remove(i_terminated);
    }
}
