use crate::{
    algorithm::{self, Algorithm},
    draw::{get_plot, save_gif},
    function::{self, Function},
    utils::ones,
};
use bit_vec::BitVec;
use std::fmt::Display;

const WIDTH: usize = 1080;
const HEIGHT: usize = 810;

fn get_dynamic_fitness<A, F>(algorithm: &A, function: &F) -> Vec<Vec<usize>>
where
    A: Algorithm + Display,
    F: Function + Display,
{
    let n = function.n();
    let populations = algorithm.trace(function);

    populations
        .into_iter()
        .map(|population| {
            let mut fitness_ones = vec![BitVec::from_elem(n, false); n + 1];

            for i in population {
                fitness_ones[i.fitness as usize].or(&i.bitvec);
            }

            fitness_ones
                .into_iter()
                .enumerate()
                .map(|(fitness, bv)| {
                    let ones = ones(&bv);
                    if ones == 0 {
                        0
                    } else {
                        ones - fitness
                    }
                })
                .collect()
        })
        .collect()
}

fn save_animation(n: usize, mu: usize) {
    let algorithm = algorithm::mu_plus_one::ConvexHullMaximization::new(mu, 0.99, 1.0 / n as f64);
    let function = function::OneMax::new(n);

    println!("Running algorithm");
    let population_ones_vec: Vec<_> = get_dynamic_fitness(&algorithm, &function);
    println!(
        "Algorithm finished in {} iterations",
        population_ones_vec.len(),
    );

    let gif_path = format!("plots/dynamic-fitness/{}-{}-speed-up.gif", n, mu,);
    let total_len = population_ones_vec.len();
    let y_max = *population_ones_vec
        .iter()
        .map(|x| x.iter().max().unwrap())
        .max()
        .unwrap();

    save_gif(
        &gif_path,
        WIDTH,
        HEIGHT,
        30,
        population_ones_vec
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i % 60 == 0)
            .map(|(i, population_ones)| {
                get_plot(
                    vec![(
                        "fitness ones - fitness",
                        population_ones
                            .into_iter()
                            .enumerate()
                            .map(|(x, y)| (x as f64, y as f64))
                            .collect(),
                    )],
                    0.0..n as f64,
                    0.0..(y_max + 1) as f64,
                    &format!("{} on {}, #{:04}/{}", algorithm, function, i, total_len),
                    WIDTH,
                    HEIGHT,
                )
                .expect("frame saving")
            }),
    );

    println!("Saving animation: {}", gif_path);
}

pub fn dynamic_fitness_experiment() {
    const N: usize = 512;
    let mus = vec![2, 9, 128];

    for mu in mus {
        save_animation(N, mu);
    }
}
