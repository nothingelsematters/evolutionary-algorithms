use std::{
    collections::BTreeMap,
    fmt::Display,
    io::{stdout, Write},
    time::Instant,
};
use tokio::runtime::Runtime;

use crate::{
    algorithm::{self, launch, Algorithm},
    function::{self, Function},
};

pub async fn rugged_runtime_experiment() {
    run().await;
}

async fn run() {
    const THREADS: usize = 4;
    const RUNS: usize = 128;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(THREADS)
        .build()
        .expect("thread pool building");

    let ns: Vec<usize> = (5..=13).map(|x| 1 << x).collect();

    // (μ + 1)

    #[allow(clippy::type_complexity)]
    let mu_getters: Vec<(&str, Box<dyn Fn(usize) -> usize>)> = vec![
        ("2", Box::new(|_: usize| 2)),
        (
            "log2(n)",
            Box::new(|x: usize| (x as f64).log2().floor() as usize),
        ),
        (
            "sqrt(n)",
            Box::new(|x: usize| (x as f64).sqrt().floor() as usize),
        ),
    ];

    for (mu_getter_name, mu_getter) in mu_getters {
        let mut mu_common_results = BTreeMap::new();
        let mut mu_chm_results = BTreeMap::new();

        for n in ns.iter().copied() {
            let function = function::RuggedOneMax::new(n);

            let mu = mu_getter(n);
            let mu_common = algorithm::mu_plus_one::Common::new(mu, 0.99, 1.0 / (n as f64));
            let mu_chm =
                algorithm::mu_plus_one::ConvexHullMaximization::new(mu, 0.99, 1.0 / (n as f64));

            mu_common_results.insert(n, run_algorithm(&runtime, mu_common, function, RUNS).await);
            mu_chm_results.insert(n, run_algorithm(&runtime, mu_chm, function, RUNS).await);
        }

        println!("{mu_getter_name}:");
        println!("mu common: {mu_common_results:#?}");
        println!("mu w/ chm: {mu_chm_results:#?}");
        println!();
    }

    // (1 + (λ, λ))

    #[allow(clippy::type_complexity)]
    let lambda_getters: Vec<(&str, Box<dyn Fn(usize) -> usize>)> = vec![
        (
            "sqrt(log2(n))",
            Box::new(|x: usize| (x as f64).log2().sqrt().floor() as usize),
        ),
        (
            "log2(n)",
            Box::new(|x: usize| (x as f64).log2().floor() as usize),
        ),
        (
            "sqrt(n)",
            Box::new(|x: usize| (x as f64).sqrt().floor() as usize),
        ),
    ];

    for (lambda_getter_name, lambda_getter) in lambda_getters {
        let mut one_plus_lambda_lambda_results = BTreeMap::new();

        for n in ns.iter().copied() {
            let function = function::RuggedOneMax::new(n);

            let lambda = lambda_getter(n);
            let one_plus_lambda_lambda = algorithm::OnePlusLambdaLambda::new(lambda, 0.1, 0.1); // TODO

            one_plus_lambda_lambda_results.insert(
                n,
                run_algorithm(&runtime, one_plus_lambda_lambda, function, RUNS).await,
            );
        }

        println!("{lambda_getter_name}:");
        println!("mu common: {one_plus_lambda_lambda_results:#?}");
        println!();
    }
}

async fn run_algorithm<A, F>(runtime: &Runtime, algorithm: A, function: F, runs: usize) -> f64
where
    A: Algorithm + Display + Send + Copy + 'static,
    F: Function + Display + Send + Copy + 'static,
{
    async fn run_task<A, F>(algorithm: A, function: F) -> usize
    where
        A: Algorithm + Send,
        F: Function + Send,
    {
        launch::runtime(&algorithm, &function)
    }

    let now = Instant::now();
    println!("Running {} on {}", algorithm, function);

    #[allow(clippy::needless_collect)]
    let tasks: Vec<_> = (0..runs)
        .map(|_i| runtime.spawn(run_task(algorithm, function)))
        .collect();

    print!("Progress: ");
    let mut runtimes = Vec::with_capacity(runs);

    for (i, task) in tasks.into_iter().enumerate() {
        print!("{i} ");
        stdout().flush().expect("stdout flush");
        runtimes.push(task.await.expect("evaluating"));
    }

    println!();
    println!("Runtimes: {runtimes:?}");

    let average = runtimes.iter().sum::<usize>() as f64 / runtimes.len() as f64;
    println!("Average: {average}");

    let elapsed = now.elapsed();
    println!("Evaluated in {elapsed:.2?}\n");

    average
}
