use crate::{
    algorithm::{self, Algorithm},
    draw::save_plot,
    function::{self, Function},
    utils::ones,
};
use std::fmt::Display;

const WIDTH: usize = 1080;
const HEIGHT: usize = 810;

fn get_metrics<A, F>(algorithm: &A, function: &F) -> Vec<(usize, usize, usize)>
where
    A: Algorithm + Display,
    F: Function + Display,
{
    let populations = algorithm.run(function);

    populations
        .into_iter()
        .map(|population| {
            let max_fitness = population
                .iter()
                .map(|x| function.fitness(x))
                .max()
                .unwrap();

            let best_individuals_ones = {
                let or = population
                    .iter()
                    .filter(|x| function.fitness(*x) == max_fitness)
                    .cloned()
                    .reduce(|a, b| {
                        let mut mutable = a;
                        mutable.or(&b);
                        mutable
                    })
                    .unwrap();

                ones(&or)
            };

            let all_ones = {
                let or = population
                    .iter()
                    .cloned()
                    .reduce(|a, b| {
                        let mut mutable = a;
                        mutable.or(&b);
                        mutable
                    })
                    .unwrap();

                ones(&or)
            };

            (max_fitness as usize, best_individuals_ones, all_ones)
        })
        .collect()
}

fn save_metrics_plot(n: usize, mu: usize) {
    let algorithm = algorithm::mu_plus_one::ConvexHullMaximization::new(mu, 0.99, 1.0 / n as f64);
    let function = function::OneMax::new(n);

    println!("Running algorithm");
    let metrics = get_metrics(&algorithm, &function);
    let iterations = metrics.len();
    println!("Algorithm finished in {} iterations", iterations);

    let metrics = vec![
        (
            "max fitness",
            metrics.iter().enumerate().map(|(i, x)| (i, x.0)).collect(),
        ),
        (
            "best ones",
            metrics.iter().enumerate().map(|(i, x)| (i, x.1)).collect(),
        ),
        (
            "all ones",
            metrics.iter().enumerate().map(|(i, x)| (i, x.2)).collect(),
        ),
    ];

    let path = format!("plots/dynamic-metrics-{}-{}.png", n, mu);
    println!("Saving plot: {}", path);

    save_plot(
        path,
        metrics,
        0..iterations,
        0..n,
        &format!("{} on {}", algorithm, function),
        WIDTH,
        HEIGHT,
    )
    .expect("frame saving");
}

pub fn dynamic_metrics_experiment() {
    const N: usize = 512;
    let mus = vec![2, 9, 128];

    for mu in mus {
        save_metrics_plot(N, mu);
    }
}
