use crate::function::Function;
use bit_vec::BitVec;
use rand::Rng;

pub mod mu_plus_one;

pub trait Algorithm {
    fn initialize(&self, function: &impl Function) -> Vec<Mutant>;

    fn stopping_criterea(population: &[Mutant], function: &impl Function) -> bool {
        population
            .last()
            .map(|x| function.is_best(&x.bitvec))
            .unwrap_or(false)
    }

    fn iterate(&self, population: &mut Vec<Mutant>, function: &impl Function);

    fn trace(&self, function: &impl Function) -> Vec<Vec<Mutant>> {
        let mut population = self.initialize(function);
        let mut trace = vec![population.clone()];

        while !Self::stopping_criterea(&population, function) {
            self.iterate(&mut population, function);
            trace.push(population.clone());
        }

        trace
    }

    fn runtime(&self, function: &impl Function) -> usize {
        let mut population = self.initialize(function);
        let mut trace = 0;

        while !Self::stopping_criterea(&population, function) {
            self.iterate(&mut population, function);
            trace += 1;
        }

        trace
    }
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
            if rng.gen::<f64>() > crossover_bias {
                x
            } else {
                y
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mutant {
    pub fitness: i64,
    pub bitvec: BitVec,
}

impl Mutant {
    fn new(bitvec: BitVec, function: &dyn Function) -> Mutant {
        Mutant {
            fitness: function.fitness(&bitvec),
            bitvec,
        }
    }
}
