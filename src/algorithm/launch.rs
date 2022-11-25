use crate::function::Function;

use super::{Algorithm, Mutant};

/// Returns every population.
pub fn trace<A, F>(algorithm: &A, function: &F) -> Vec<Vec<Mutant>>
where
    A: Algorithm,
    F: Function,
{
    let mut population = algorithm.initialize(function);
    let mut trace = vec![population.clone()];

    while !A::stopping_criterea(&population, function) {
        algorithm.iterate(&mut population, function);
        trace.push(population.clone());
    }

    trace
}

/// Returns number of iterations.
pub fn runtime<A, F>(algorithm: &A, function: &F) -> usize
where
    A: Algorithm,
    F: Function,
{
    let mut population = algorithm.initialize(function);
    let mut trace = 0;

    while !A::stopping_criterea(&population, function) {
        algorithm.iterate(&mut population, function);
        trace += 1;
    }

    trace
}
