use std::{collections::BTreeMap, fmt::Display, time::Instant};

use crate::{
    algorithm::{self, Algorithm},
    function::{self, Function},
};

pub async fn rugged_runtime_experiment() {
    run().await;
}

async fn run_task(algorithm: impl Algorithm, function: impl Function) -> usize {
    algorithm.runtime(&function)
}

async fn run() {
    const RUNS: usize = 128;

    #[allow(clippy::type_complexity)]
    let mu_getters: Vec<(&str, Box<dyn Fn(usize) -> usize>)> = vec![
        ("2", Box::new(|_: usize| 2)),
        (
            "log2",
            Box::new(|x: usize| (x as f64).log2().floor() as usize),
        ),
        (
            "sqrt",
            Box::new(|x: usize| (x as f64).sqrt().floor() as usize),
        ),
    ];

    let ns: Vec<usize> = (5..=6).map(|x| 1 << x).collect();

    for (mu_getter_name, mu_getter) in mu_getters {
        let mut mu_common_results = BTreeMap::new();
        let mut mu_chm_results = BTreeMap::new();

        for n in ns.iter().copied() {
            let function = function::RuggedOneMax::new(n);

            let mu = mu_getter(n);
            let mu_common = algorithm::mu_plus_one::Common::new(mu, 0.99, 1.0 / (n as f64));
            let mu_chm =
                algorithm::mu_plus_one::ConvexHullMaximization::new(mu, 0.99, 1.0 / (n as f64));
            // TODO (1 + (l, l))

            mu_common_results.insert(n, run_algorithm(mu_common, function, RUNS).await);
            mu_chm_results.insert(n, run_algorithm(mu_chm, function, RUNS).await);
        }

        println!("{mu_getter_name}:");
        println!("mu common: {mu_common_results:#?}");
        println!("mu w/ chm common: {mu_chm_results:#?}");
        println!();
    }
}

async fn run_algorithm<A, B>(algorithm: A, function: B, runs: usize) -> f64
where
    A: Algorithm + Display + Send + Copy + 'static,
    B: Function + Display + Send + Copy + 'static,
{
    let now = Instant::now();
    println!("Running {} on {}", algorithm, function);

    let tasks: Vec<_> = (0..runs)
        .map(|_i| tokio::task::spawn(run_task(algorithm, function)))
        .collect();

    let mut runtimes = Vec::with_capacity(runs);

    for task in tasks {
        runtimes.push(task.await.expect("evaluating"));
    }
    println!("Runtimes: {runtimes:?}");

    let average = runtimes.iter().sum::<usize>() as f64 / runtimes.len() as f64;
    println!("Average: {}", average);

    let elapsed = now.elapsed();
    println!("Evaluated in {:.2?}\n", elapsed);

    average
}
