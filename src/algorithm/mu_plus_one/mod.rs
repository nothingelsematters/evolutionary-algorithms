use super::{crossover, mutate, Mutant};
use crate::function::Function;
use bit_vec::BitVec;
use rand::prelude::*;

mod common;
mod convex_hull_maximization;

pub use common::Common;
pub use convex_hull_maximization::ConvexHullMaximization;

fn get_random(mutants: &[Mutant]) -> &BitVec {
    &mutants
        .choose(&mut rand::thread_rng())
        .expect("non-empty population")
        .bitvec
}

fn mu_plus_one_iterate<F, BT>(
    crossover_probability: f64,
    mutation_rate: f64,
    function: &F,
    population: &mut Vec<Mutant>,
    break_ties: BT,
) where
    F: Function,
    BT: Fn(usize, &mut Vec<Mutant>),
{
    let p: f64 = rand::random();
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
