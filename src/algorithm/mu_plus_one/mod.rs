use super::{crossover, mutate, Mutant};
use crate::function::Function;
use bit_vec::BitVec;
use rand::prelude::*;

mod common;
mod convex_hull_maximization;

pub use common::Common;
pub use convex_hull_maximization::ConvexHullMaximization;

fn initialize(mu: usize, function: &dyn Function) -> Vec<Mutant> {
    let mut population = Vec::with_capacity(mu);

    let n = function.n();
    for _ in 0..mu {
        let bitvec = (0..n).map(|_| rand::random()).collect();
        population.push(Mutant::new(bitvec, function));
    }

    population
}

fn get_random(mutants: &[Mutant]) -> &BitVec {
    let i = rand::random::<usize>() % mutants.len();
    &mutants[i].bitvec
}

fn mu_plus_one_iterate<F>(
    crossover_probability: f64,
    mutation_rate: f64,
    function: &impl Function,
    population: &mut Vec<Mutant>,
    break_ties: F,
) where
    F: Fn(usize, &mut Vec<Mutant>),
{
    let p: f64 = rand::thread_rng().gen();
    let x = get_random(population);

    let z = if p <= crossover_probability {
        let y = get_random(population);
        mutate(&crossover(x, y, 0.5), mutation_rate)
    } else {
        mutate(x, mutation_rate)
    };

    population.push(Mutant::new(z, function));
    break_ties(function.n(), population);
}
