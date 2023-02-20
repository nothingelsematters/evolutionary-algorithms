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

pub fn get_fitness_evaluations_with_n3_time_limit<A, F>(
    algorithm: &A,
    function: &F,
) -> Option<usize>
where
    A: Algorithm,
    F: Function,
{
    let iterations = {
        let mut population = algorithm.initialize(function);
        let mut iterations = 0;
        let n3 = (function.n() as u64).pow(3);

        while iterations <= n3 && !A::stopping_criterea(&population, function) {
            algorithm.iterate(&mut population, function);
            iterations += 1;
        }

        if iterations > n3 {
            return None;
        }

        iterations
    };

    Some(algorithm.fitness_evaluations(iterations as usize))
}
