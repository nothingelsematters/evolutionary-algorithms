use crate::function::Function;

use super::{Algorithm, Mutant};

pub fn get_trace<A, F>(algorithm: &A, function: &F) -> Vec<Vec<Mutant>>
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

pub fn get_iterations<A, F>(algorithm: &A, function: &F) -> usize
where
    A: Algorithm,
    F: Function,
{
    let mut population = algorithm.initialize(function);
    let mut iterations = 0;

    while !A::stopping_criterea(&population, function) {
        algorithm.iterate(&mut population, function);
        iterations += 1;
    }

    iterations
}

pub fn get_fitness_evaluations<A, F>(algorithm: &A, function: &F) -> usize
where
    A: Algorithm,
    F: Function,
{
    algorithm.fitness_evaluations(get_iterations(algorithm, function))
}
