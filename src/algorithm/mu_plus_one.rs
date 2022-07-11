use super::{crossover, mutate, Algorithm, Mutant};
use crate::{function::Function, COUNT, DRIFT};
use bit_vec::BitVec;
use rand::prelude::*;
use std::collections::BinaryHeap;

fn initialize(mu: usize, function: &dyn Function) -> BinaryHeap<Mutant> {
    let mut population = BinaryHeap::with_capacity(mu);

    let n = function.n();
    for _ in 0..mu {
        let bitvec = (0..n).map(|_| rand::random()).collect();
        population.push(Mutant::new(bitvec, function));
    }

    population
}

fn get_random(heap: &BinaryHeap<Mutant>) -> &BitVec {
    let i = rand::random::<usize>() % heap.len();
    &heap.iter().nth(i).unwrap().bitvec
}

fn mu_plus_one<F>(
    mu: usize,
    crossover_probability: f64,
    mutation_rate: f64,
    function: impl Function,
    break_ties: F,
) where
    F: Fn(usize, &mut BinaryHeap<Mutant>),
{
    let mut population = initialize(mu, &function);

    // let mut bit_counts = vec![0usize; function.n()];
    loop {
        // Run analysis block
        {
            let mut new_bit_counts = vec![0usize; function.n()];
            for mutant in population.iter() {
                (0..function.n()).for_each(|i| {
                    new_bit_counts[i] += if mutant.bitvec[i] { 1 } else { 0 };
                });
            }

            let mut count = COUNT.lock().unwrap();
            // for i in new_bit_counts.iter() {
            //     count[*i] += 1;
            // }

            let zero = new_bit_counts.iter().filter(|x| **x == 0).count();
            let fitness = population
                .iter()
                .map(|x| function.fitness(&x.bitvec))
                .max()
                .unwrap();
            count[fitness as usize] += 1;

            // let mut drift = DRIFT.lock().unwrap();
            // (0..mu).for_each(|i| {
            //     drift[i] += new_bit_counts[i] as i64 - bit_counts[i] as i64;
            // });

            // bit_counts = new_bit_counts;
            // if *count.iter().take(mu).min().unwrap() >= 1000 {
            //     break;
            // }
        }

        let p: f64 = rand::thread_rng().gen();
        let x = get_random(&population);

        let z = if p <= crossover_probability {
            let y = get_random(&population);
            mutate(&crossover(x, y, 0.5), mutation_rate)
        } else {
            mutate(x, mutation_rate)
        };

        // TODO temporary stopping criteria: reaching the local optimum
        if function.is_local_optimum(&z) || function.is_best(&z) {
            break;
        }

        population.push(Mutant::new(z, &function));
        break_ties(function.n(), &mut population);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Common {
    mu: usize,
    crossover_probability: f64,
    mutation_rate: f64,
}

impl Common {
    pub fn new(mu: usize, crossover_probability: f64, mutation_rate: f64) -> Common {
        Common {
            mu,
            crossover_probability,
            mutation_rate,
        }
    }
}

impl Algorithm for Common {
    fn run(&self, function: impl Function) {
        mu_plus_one(
            self.mu,
            self.crossover_probability,
            self.mutation_rate,
            function,
            |_n, population| {
                population.pop();
            },
        )
    }
}

#[derive(Clone, Copy)]
pub struct ConvexHullMaximization {
    mu: usize,
    crossover_probability: f64,
    mutation_rate: f64,
}

impl Algorithm for ConvexHullMaximization {
    fn run(&self, function: impl Function) {
        mu_plus_one(
            self.mu,
            self.crossover_probability,
            self.mutation_rate,
            function,
            ConvexHullMaximization::break_ties,
        )
    }
}

impl ConvexHullMaximization {
    pub fn new(
        mu: usize,
        crossover_probability: f64,
        mutation_rate: f64,
    ) -> ConvexHullMaximization {
        ConvexHullMaximization {
            mu,
            crossover_probability,
            mutation_rate,
        }
    }

    fn break_ties(n: usize, population: &mut BinaryHeap<Mutant>) {
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
                for (j, choosing_j) in choosing.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    different = choosing_j
                        .bitvec
                        .iter()
                        .zip(different)
                        .map(|(bit, value)| value | (if bit { 0b10 } else { 0b1 }))
                        .collect();
                }

                let value: usize = different
                    .iter()
                    .map(|x| match x {
                        1 | 2 => 1,
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
