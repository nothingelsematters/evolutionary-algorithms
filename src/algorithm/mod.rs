use crate::function::Function;
use bit_vec::BitVec;
use rand::Rng;

pub mod launch;
pub mod mu_plus_one;
pub mod one_plus_lambda_lambda;

pub use one_plus_lambda_lambda::OnePlusLambdaLambda;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mutant {
    pub bitvec: BitVec,
    pub fitness: i64,
}

impl Mutant {
    fn new<F: Function>(bitvec: BitVec, function: &F) -> Mutant {
        Mutant {
            fitness: function.fitness(&bitvec),
            bitvec,
        }
    }
}

pub trait Algorithm {
    fn initialize<F: Function>(&self, function: &F) -> Vec<Mutant>;

    fn iterate<F: Function>(&self, population: &mut Vec<Mutant>, function: &F);

    fn stopping_criterea<F: Function>(population: &[Mutant], function: &F) -> bool {
        let best_fitness = function.best_fitness();
        population.iter().any(|x| x.fitness == best_fitness)
    }
}

pub fn initialize_random<F: Function>(population_size: usize, function: &F) -> Vec<Mutant> {
    (0..population_size)
        .map(|_| {
            let bitvec = (0..function.n()).map(|_| rand::random()).collect();
            Mutant::new(bitvec, function)
        })
        .collect()
}

fn mutate(bitvec: &BitVec, mutation_rate: f64) -> BitVec {
    let mut rng = rand::thread_rng();

    bitvec
        .iter()
        .map(|x| {
            if rng.gen::<f64>() < mutation_rate {
                !x
            } else {
                x
            }
        })
        .collect()
}

fn crossover(left: &BitVec, right: &BitVec, crossover_bias: f64) -> BitVec {
    let mut rng = rand::thread_rng();

    left.iter()
        .zip(right)
        .map(|(x, y)| {
            if rng.gen::<f64>() <= crossover_bias {
                x
            } else {
                y
            }
        })
        .collect()
}
