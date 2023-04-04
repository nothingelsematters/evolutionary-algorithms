use futures::future::join_all;
use std::{
    fmt::{Debug, Display},
    io::{stdout, Write},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::runtime::Runtime;

use super::{runtime, RUNS, THREADS};
use crate::{
    algorithm::{self, mu_plus_one::Common, Algorithm},
    function::{self, Function, RuggedOneMax},
};

#[tokio::test]
async fn rugged_optimal_parameters() {
    async fn run_algorithm<A, F>(runtime: &Runtime, algorithm: A, function: F)
    where
        A: Algorithm + Display + Send + Copy + 'static,
        F: Function + Display + Send + Copy + 'static,
    {
        crate::experiment::run_algorithm_with_tl(runtime, algorithm, function, RUNS, THREADS).await
    }

    let runtime = runtime();

    let log_n: usize = 15;
    let n: usize = 1 << log_n;
    let population_sizes: Vec<usize> = (1..log_n as u32).map(|x| 1 << x).collect();

    let function = function::RuggedOneMax::new(2, n);

    for population_size in population_sizes {
        let mu_common = algorithm::mu_plus_one::ConvexHullMaximization::new(
            population_size,
            0.99,
            1.0 / (n as f64),
        );
        run_algorithm(&runtime, mu_common, function).await;

        // let one_plus_lambda_lambda = algorithm::OnePlusLambdaLambda::new(
        //     population_size,
        //     population_size as f64 / n as f64,
        //     1.0 / population_size as f64,
        // );

        // run_algorithm(&runtime, one_plus_lambda_lambda, function).await;
    }
    println!();
}

#[tokio::test]
async fn rugged_inspection() {
    async fn run_task<F, L, T>(
        algorithm: Common,
        function: F,
        progress_counter: Arc<Mutex<(usize, Duration)>>,
        launcher: L,
    ) -> T
    where
        F: Function + Send,
        L: Fn(&Common, &F) -> T,
        T: Debug,
    {
        let now = Instant::now();
        let result = launcher(&algorithm, &function);
        let elapsed = now.elapsed();

        let mut guard = progress_counter.lock().expect("mutex locking");
        guard.0 += 1;
        guard.1 += elapsed;

        println!("{result:?}");
        println!("{} ({})", guard.0 - 1, algorithm.mu);

        stdout().flush().expect("stdout flush");

        result
    }

    async fn run_algorithm(runtime: &Runtime, algorithm: Common, f: RuggedOneMax) {
        let now = Instant::now();
        println!("Running {} on {}", algorithm, f);

        let progress_counter = Arc::new(Mutex::new((0, Duration::new(0, 0))));

        print!("Progress: ");
        join_all((0..8).map(|_i| {
            runtime.spawn(run_task(algorithm, f, progress_counter.clone(), |a, f| {
                a.launch_with_fitness_inspection(f)
            }))
        }))
        .await
        .into_iter()
        .for_each(|x| {
            x.expect("join");
        });

        let elapsed = now.elapsed();
        println!("Evaluated in {elapsed:.2?}\n");
    }

    let runtime = runtime();

    let log_n: usize = 15;
    let n: usize = 1 << log_n;
    let population_sizes: Vec<usize> = [6, 12, 9, 4, 15].into_iter().map(|x| 1 << x).collect();

    let function = function::RuggedOneMax::new(2, n);

    for population_size in population_sizes {
        let mu_common =
            algorithm::mu_plus_one::Common::new(population_size, 0.99, 1.0 / (n as f64));
        run_algorithm(&runtime, mu_common, function).await;
    }
    println!();
}
