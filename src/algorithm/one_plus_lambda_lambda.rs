use std::fmt::Display;

use rand::seq::IteratorRandom;

use super::{crossover, initialize_random, Algorithm, Mutant};
use crate::function::Function;

pub struct OnePlusLambdaLambda {
    lambda: usize,
    crossover_bias: f64,
}

impl Display for OnePlusLambdaLambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(1 + (λ, λ)), λ = {}, c = {}",
            self.lambda, self.crossover_bias,
        )
    }
}

impl Algorithm for OnePlusLambdaLambda {
    fn initialize<F: Function>(&self, function: &F) -> Vec<Mutant> {
        initialize_random(1, function)
    }

    fn iterate<F: Function>(&self, population: &mut Vec<Mutant>, function: &F) {
        let x = population.first().unwrap();

        // mutation phase

        let l = 1; // TODO

        let x_dash = (0..self.lambda)
            .map(|_| {
                let mut x_dash = x.clone();
                let indices = (0..x.bitvec.len()).choose_multiple(&mut rand::thread_rng(), l);

                for i in indices {
                    x_dash.bitvec.set(i, !x_dash.bitvec[i]);
                }

                x_dash
            })
            .max_by_key(|x_dash| x_dash.fitness)
            .unwrap();

        // crossover phase

        let y = (0..self.lambda)
            .map(|_| {
                let y_bitvec = crossover(&x_dash.bitvec, &x.bitvec, self.crossover_bias);
                Mutant::new(y_bitvec, function)
            })
            .max_by_key(|y| y.fitness)
            .unwrap();

        if y.fitness >= x.fitness {
            population[0] = y;
        }
    }
}
