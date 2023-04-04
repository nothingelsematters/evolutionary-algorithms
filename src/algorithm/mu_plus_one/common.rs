use rand::seq::SliceRandom;

use crate::{
    algorithm::{crossover, initialize_random, mutate, Algorithm, Mutant},
    function::Function,
};
use std::{cmp::max, fmt::Display};

#[derive(Debug, Copy, Clone)]
pub struct Common {
    pub mu: usize,
    pub crossover_probability: f64,
    pub mutation_rate: f64,
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

    fn fitness_evaluations(&self, iterations: usize) -> usize {
        iterations + self.mu
    }

    fn iterate<F: Function>(&self, population: &mut Vec<Mutant>, function: &F) {
        super::mu_plus_one_iterate(
            self.crossover_probability,
            self.mutation_rate,
            function,
            population,
            |_n, population| {
                let min_fitness = population.iter().map(|x| x.fitness).min().unwrap();
                let min_indices: Vec<usize> = population
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| x.fitness == min_fitness)
                    .map(|(x, _)| x)
                    .collect();
                let remove_index = *min_indices.choose(&mut rand::thread_rng()).unwrap();

                population.swap_remove(remove_index);

                let len = population.len();
                population.swap(remove_index, len - 1);
            },
        )
    }

    fn stopping_criterea<F: Function>(population: &[Mutant], function: &F) -> bool {
        function.best_fitness() <= population.last().unwrap().fitness
    }
}

impl Common {
    pub fn launch_with_fitness_inspection<F: Function>(&self, function: &F) -> Vec<(i64, i64)> {
        let mut population = self.initialize(function);

        let mut best_fitness = population.iter().map(|x| x.fitness).max().unwrap();
        let mut best_crossover = population
            .iter()
            .enumerate()
            .flat_map(|(i, x)| {
                population
                    .iter()
                    .enumerate()
                    .filter(move |(j, _)| i != *j)
                    .map(move |(_, y)| (x, y))
            })
            .map(|(x, y)| {
                let mut clone = x.bitvec.clone();
                clone.or(&y.bitvec);
                function.fitness(&clone)
            })
            .max()
            .unwrap();

        let mut results = vec![(best_fitness, best_crossover)];

        while best_fitness < function.best_fitness() {
            let p: f64 = rand::random();
            let x = super::get_random(&population);

            let z = if p <= self.crossover_probability {
                let y = super::get_random(&population);
                mutate(&crossover(x, y, 0.5), self.mutation_rate)
            } else {
                mutate(x, self.mutation_rate)
            };

            let new_mutant = Mutant::new(z, function);

            population.push(new_mutant.clone());

            // breaking ties
            let min_fitness = population.iter().map(|x| x.fitness).min().unwrap();
            let min_indices: Vec<usize> = population
                .iter()
                .enumerate()
                .filter(|(_, x)| x.fitness == min_fitness)
                .map(|(x, _)| x)
                .collect();
            let remove_index = *min_indices.choose(&mut rand::thread_rng()).unwrap();

            population.swap_remove(remove_index);

            // updating results
            if remove_index != population.len() {
                best_fitness = max(best_fitness, new_mutant.fitness);
                let new_best_crossover = population
                    .iter()
                    .take(population.len() - 1)
                    .map(|x| {
                        let mut clone = x.bitvec.clone();
                        clone.or(&new_mutant.bitvec);
                        function.fitness(&clone)
                    })
                    .max()
                    .unwrap();
                best_crossover = max(best_crossover, new_best_crossover);
                results.push((best_fitness, best_crossover));
            }
        }

        results
    }
}
