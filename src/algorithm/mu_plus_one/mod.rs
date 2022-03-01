use super::{crossover, mutate};
use crate::function::Function;
use bit_vec::BitVec;
use lazy_static::lazy_static;
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

pub mod common;
pub mod convex_hull_maximization;

pub use common::Common;
pub use convex_hull_maximization::ConvexHullMaximization;

// a crutch for multi item heap
lazy_static! {
    pub static ref MUTANT_NUMBER: AtomicUsize = AtomicUsize::new(0);
}

#[derive(PartialEq, Eq)]
pub struct Mutant {
    fitness: i64,
    bitvec: BitVec,
    crutch: usize,
}

impl PartialOrd for Mutant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // a crutch for a min heap
        let result = match self.fitness.cmp(&other.fitness) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => (&self.bitvec, self.crutch).cmp(&(&other.bitvec, other.crutch)),
        };
        Some(result)
    }
}

impl Ord for Mutant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Mutant {
    fn new(bitvec: BitVec, function: &dyn Function) -> Mutant {
        Mutant {
            fitness: function.fitness(&bitvec),
            bitvec,
            crutch: MUTANT_NUMBER.fetch_add(1, AtomicOrdering::SeqCst),
        }
    }
}

fn initialize(mu: usize, function: &dyn Function) -> BinaryHeap<Mutant> {
    let mut population = BinaryHeap::with_capacity(mu);

    let n = function.n();
    for _ in 0..mu {
        let bitvec = (0..n).map(|_| rand::random()).collect();
        population.push(Mutant::new(bitvec, function));
    }

    population
}

fn get_random(queue: &BinaryHeap<Mutant>) -> &BitVec {
    let i = rand::random::<usize>() % queue.len();
    &queue.iter().nth(i).unwrap().bitvec
}

pub trait MuPlusOne {
    fn mu(&self) -> usize;

    fn crossover_probability(&self) -> f64;

    fn mutation_rate(&self) -> f64;

    fn break_ties(&self, n: usize, population: &mut BinaryHeap<Mutant>);

    fn run(&self, function: &dyn Function) {
        let mut population = initialize(self.mu(), function);

        let mut set = HashSet::new();
        loop {
            // Run analysis block
            {
                let max_fitness = population.iter().map(|x| x.fitness).max().unwrap();
                if !set.contains(&max_fitness) {
                    set.insert(max_fitness);
                    let mut guard = crate::MAP.lock().unwrap();
                    let entry = guard.entry(max_fitness).or_insert((0, 0));
                    let positions = population.iter().fold(
                        (0..function.n()).map(|_| false).collect::<BitVec>(),
                        |a, b| a.iter().zip(b.bitvec.iter()).map(|(x, y)| x || y).collect(),
                    );
                    *entry = (
                        entry.0 + 1,
                        entry.1 + positions.iter().filter(|x| *x).count(),
                    );
                }
            }

            let p: f64 = rand::thread_rng().gen();
            let x = get_random(&population);

            let z = if p <= self.crossover_probability() {
                let y = get_random(&population);
                mutate(&crossover(x, y, 0.5), self.mutation_rate())
            } else {
                mutate(x, self.mutation_rate())
            };

            // TODO temporary stopping criteria: reaching the local optimum
            if function.is_local_optimum(&z) {
                break;
            }

            population.push(Mutant::new(z, function));

            self.break_ties(function.n(), &mut population);
        }
    }
}
