use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

use bit_vec::BitVec;
use rand::prelude::*;

use super::{Algorithm, EvolutionaryAlgorithm, GeneticAlgorithm};
use crate::function::Function;

// a crutch for multi item heap
static mut MUTANT_NUMBER: usize = 0;

#[derive(PartialEq, Eq)]
struct Mutant {
    fitness: usize,
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
    fn new(bitvec: BitVec, function: &Box<dyn Function>) -> Mutant {
        unsafe {
            MUTANT_NUMBER += 1;

            Mutant {
                fitness: function.fitness(&bitvec),
                bitvec,
                crutch: MUTANT_NUMBER,
            }
        }
    }
}

pub struct MuPlusOne {
    mu: usize,
    crossover_probability: f64,
    mutation_rate: f64,
}

fn in_one(float: f64) -> bool {
    0.0 <= float && float <= 1.0
}

impl MuPlusOne {
    pub fn new(
        mu: usize,
        crossover_probability: f64,
        mutation_rate: f64,
    ) -> Result<MuPlusOne, String> {
        if in_one(crossover_probability) && in_one(mutation_rate) {
            Ok(MuPlusOne {
                mu,
                crossover_probability,
                mutation_rate,
            })
        } else {
            Err(String::from(
                "crossover_probability, mutation_rate, crossover_bias must be in 0.0..=1.0",
            ))
        }
    }
}

impl GeneticAlgorithm for MuPlusOne {
    fn crossover_bias(&self) -> f64 {
        0.5
    }
}

impl EvolutionaryAlgorithm for MuPlusOne {
    fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }
}

impl Algorithm for MuPlusOne {
    fn run(&self, function: Box<dyn Function>) -> usize {
        let mut population = self.initialize(&function);
        // let mut diversity = Vec::new();

        let mut minimum_unique_population = usize::max_value();

        loop {
            let p: f64 = rand::thread_rng().gen();
            let x = MuPlusOne::get_random(&population);

            let z = if p <= self.crossover_probability {
                let y = MuPlusOne::get_random(&population);
                self.mutate(&self.crossover(x, y))
            } else {
                self.mutate(x)
            };

            // TODO temporary stopping criteria: reaching the local optimum
            if function.is_local_optimum(&z) {
                break;
            }

            population.push(Mutant::new(z, &function));
            println!(
                "{}  {:?}",
                minimum_unique_population,
                population
                    .iter()
                    .map(|x| function.fitness(&x.bitvec))
                    .collect::<Vec<_>>()
            );

            MuPlusOne::convex_hull_maximization_elimination(function.n(), &mut population);

            minimum_unique_population = minimum_unique_population.min(
                population
                    .iter()
                    .map(|x| &x.bitvec)
                    .collect::<HashSet<&BitVec>>()
                    .len(),
            );
        }

        minimum_unique_population
    }
}

impl MuPlusOne {
    fn initialize(&self, function: &Box<dyn Function>) -> BinaryHeap<Mutant> {
        let mut population = BinaryHeap::with_capacity(self.mu);

        let n = function.n();
        for _ in 0..self.mu {
            let bitvec = (0..n).map(|_| rand::random()).collect();
            population.push(Mutant::new(bitvec, function));
        }

        population
    }

    fn get_random(queue: &BinaryHeap<Mutant>) -> &BitVec {
        let i = rand::random::<usize>() % queue.len();
        &queue.iter().nth(i).unwrap().bitvec
    }

    fn convex_hull_maximization_elimination(n: usize, population: &mut BinaryHeap<Mutant>) {
        let mut choosing = vec![population.pop().unwrap()];
        let fitness = choosing[0].fitness;
        while !population.is_empty() && population.peek().unwrap().fitness == fitness {
            choosing.push(population.pop().unwrap());
        }

        let different = population.iter().fold(vec![0u8; n], |init, m| {
            m.bitvec
                .iter()
                .zip(init)
                .map(|(bit, value)| value | (if bit { 0b10 } else { 0b1 }))
                .collect()
        });

        let i_terminated = (0..choosing.len())
            .map(|i| {
                let mut different = different.clone();
                for j in 0..choosing.len() {
                    if i == j {
                        continue;
                    }
                    different = choosing[j]
                        .bitvec
                        .iter()
                        .zip(different)
                        .map(|(bit, value)| value | (if bit { 0b10 } else { 0b1 }))
                        .collect();
                }

                let value: usize = different
                    .iter()
                    .map(|x| match x {
                        1..=2 => 1,
                        3 => 2,
                        _ => panic!(),
                    })
                    .sum();
                (i, value)
            })
            .min_by(|(_, x_value), (_, y_value)| x_value.cmp(y_value))
            .unwrap()
            .0;

        for (i, mutant) in choosing.into_iter().enumerate() {
            if i == i_terminated {
                continue;
            }
            population.push(mutant);
        }
    }
}
