use futures::future::join_all;
use std::{
    fmt::Display,
    io::{stdout, Write},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::runtime::Runtime;

use crate::{
    algorithm::{self, launch, Algorithm},
    function::{self, Function},
};

const THREADS: usize = 6;
const RUNS: usize = 128;

#[tokio::test]
async fn rugged_runtime() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(THREADS)
        .build()
        .expect("thread pool building");

    let ns: Vec<usize> = (16..=16).map(|x| 1 << x).collect();

    #[allow(clippy::type_complexity)]
    let mu_getters: Vec<(&str, Box<dyn Fn(usize) -> usize>)> = vec![
        // ("2", Box::new(|_: usize| 2)),
        // ("log2(n)", Box::new(|x: usize| (x as f64).log2() as usize)),
        ("sqrt(n)", Box::new(|x: usize| (x as f64).sqrt() as usize)),
    ];

    // #[allow(clippy::type_complexity)]
    // let lambda_getters: Vec<(&str, Box<dyn Fn(usize) -> usize>)> = vec![
    //     (
    //         "sqrt(log2(n))",
    //         Box::new(|x: usize| (x as f64).log2().sqrt() as usize),
    //     ),
    //     ("log2(n)", Box::new(|x: usize| (x as f64).log2() as usize)),
    //     ("sqrt(n)", Box::new(|x: usize| (x as f64).sqrt() as usize)),
    // ];

    for n in ns {
        let function = function::RuggedOneMax::new(3, n);

        for (mu_getter_name, mu_getter) in mu_getters.iter() {
            let mu = mu_getter(n);

            let mu_common = algorithm::mu_plus_one::Common::new(mu, 0.99, 1.0 / (n as f64));
            println!("{mu_getter_name}");
            run_algorithm(&runtime, mu_common, function, RUNS).await;

            // let mu_chm =
            //     algorithm::mu_plus_one::ConvexHullMaximization::new(mu, 0.99, 1.0 / (n as f64));
            // println!("{mu_getter_name}");
            // run_algorithm(&runtime, mu_chm, function, RUNS).await;
        }
        println!();

        // for (lambda_getter_name, lambda_getter) in lambda_getters.iter() {
        //     let lambda = lambda_getter(n);
        //     let one_plus_lambda_lambda = algorithm::OnePlusLambdaLambda::new(
        //         lambda,
        //         lambda as f64 / function.n() as f64,
        //         1.0 / lambda as f64,
        //     );

        //     println!("{lambda_getter_name}");
        //     run_algorithm(&runtime, one_plus_lambda_lambda, function, RUNS).await;
        // }
        // println!();
    }
}

async fn run_algorithm<A, F>(runtime: &Runtime, algorithm: A, function: F, runs: usize) -> f64
where
    A: Algorithm + Display + Send + Copy + 'static,
    F: Function + Display + Send + Copy + 'static,
{
    async fn run_task<A, F>(
        algorithm: A,
        function: F,
        progress_counter: Arc<Mutex<(usize, Duration)>>,
    ) -> usize
    where
        A: Algorithm + Send,
        F: Function + Send,
    {
        let now = Instant::now();
        let result = launch::get_fitness_evaluations(&algorithm, &function);
        let elapsed = now.elapsed();

        let mut guard = progress_counter.lock().expect("mutex locking");
        guard.0 += 1;
        guard.1 += elapsed;
        let left = ((RUNS - guard.0) as f64 / THREADS as f64)
            * (guard.1.as_secs() as f64 / guard.0 as f64);
        print!(
            "{} ({:?} s left) ",
            guard.0 - 1,
            Duration::new(left as u64, 0),
        );
        stdout().flush().expect("stdout flush");

        result
    }

    let now = Instant::now();
    println!("Running {} on {}", algorithm, function);

    let progress_counter = Arc::new(Mutex::new((0, Duration::new(0, 0))));

    print!("Progress: ");
    let runtimes = join_all(
        (0..runs).map(|_i| runtime.spawn(run_task(algorithm, function, progress_counter.clone()))),
    )
    .await
    .into_iter()
    .map(|x| x.expect("join"))
    .collect::<Vec<_>>();

    println!();
    println!("Runtimes: {runtimes:?}");
    let average = runtimes.into_iter().sum::<usize>() as f64 / runs as f64;
    println!("Average: {average}");

    let elapsed = now.elapsed();
    println!("Evaluated in {elapsed:.2?}\n");

    average
}
