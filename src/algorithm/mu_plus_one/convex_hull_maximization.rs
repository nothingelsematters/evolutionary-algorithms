use super::{MuPlusOne, Mutant};
use std::collections::BinaryHeap;

pub struct ConvexHullMaximization {
    mu: usize,
    crossover_probability: f64,
    mutation_rate: f64,
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
}

impl MuPlusOne for ConvexHullMaximization {
    fn mu(&self) -> usize {
        self.mu
    }

    fn crossover_probability(&self) -> f64 {
        self.crossover_probability
    }

    fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    fn break_ties(&self, n: usize, population: &mut BinaryHeap<Mutant>) {
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
