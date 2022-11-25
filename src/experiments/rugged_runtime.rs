use std::{collections::BTreeMap, fmt::Display, io::Write, time::Instant};
use tokio::runtime::Runtime;

use crate::{
    algorithm::{self, launch::runtime, Algorithm},
    function::{self, Function},
};

pub async fn rugged_runtime_experiment() {
    run().await;
}

async fn run_task<A, F>(algorithm: A, function: F) -> usize
where
    A: Algorithm + Send,
    F: Function + Send,
{
    runtime(&algorithm, &function)
}

async fn run() {
    const THREADS: usize = 4;
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

    let ns: Vec<usize> = (5..=13).map(|x| 1 << x).collect();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(THREADS)
        .build()
        .expect("thread pool building");

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

            mu_common_results.insert(n, run_algorithm(&runtime, mu_common, function, RUNS).await);
            mu_chm_results.insert(n, run_algorithm(&runtime, mu_chm, function, RUNS).await);
        }

        println!("{mu_getter_name}:");
        println!("mu common: {mu_common_results:#?}");
        println!("mu w/ chm: {mu_chm_results:#?}");
        println!();
    }
}

async fn run_algorithm<A, F>(runtime: &Runtime, algorithm: A, function: F, runs: usize) -> f64
where
    A: Algorithm + Display + Send + Copy + 'static,
    F: Function + Display + Send + Copy + 'static,
{
    let now = Instant::now();
    println!("Running {} on {}", algorithm, function);

    #[allow(clippy::needless_collect)]
    let tasks: Vec<_> = (0..runs)
        .map(|_i| runtime.spawn(run_task(algorithm, function)))
        .collect();

    let mut runtimes = Vec::with_capacity(runs);

    print!("Progress: ");
    for (i, task) in tasks.into_iter().enumerate() {
        print!("{i} ");
        std::io::stdout().flush().expect("stdout flush");
        runtimes.push(task.await.expect("evaluating"));
    }
    println!();
    println!("Runtimes: {runtimes:?}");

    let average = runtimes.iter().sum::<usize>() as f64 / runtimes.len() as f64;
    println!("Average: {}", average);

    let elapsed = now.elapsed();
    println!("Evaluated in {:.2?}\n", elapsed);

    average
}
