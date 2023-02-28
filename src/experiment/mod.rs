use std::{
    fmt::{Debug, Display},
    io::{stdout, Write},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use futures::future::join_all;
use tokio::{runtime::Runtime, task::JoinSet};

use crate::{
    algorithm::{launch, Algorithm},
    function::Function,
};

mod dynamic_fitness;
mod dynamic_metrics;
mod rugged_runtime;
mod runtime;

async fn run_task<A, F, L, T>(
    algorithm: A,
    function: F,
    progress_counter: Arc<Mutex<(usize, Duration)>>,
    runs: usize,
    threads: usize,
    launcher: L,
) -> T
where
    A: Algorithm + Send,
    F: Function + Send,
    L: Fn(&A, &F) -> T,
    T: Debug,
{
    let now = Instant::now();
    let result = launcher(&algorithm, &function);
    let elapsed = now.elapsed();

    let mut guard = progress_counter.lock().expect("mutex locking");
    guard.0 += 1;
    guard.1 += elapsed;
    let left =
        ((runs - guard.0) as f64 / threads as f64) * (guard.1.as_secs() as f64 / guard.0 as f64);
    print!(
        "{} ({result:?}; {:?} s left) ",
        guard.0 - 1,
        Duration::new(left as u64, 0),
    );

    stdout().flush().expect("stdout flush");

    result
}

async fn run_algorithm<A, F>(
    runtime: &Runtime,
    algorithm: A,
    function: F,
    runs: usize,
    threads: usize,
) -> f64
where
    A: Algorithm + Display + Send + Copy + 'static,
    F: Function + Display + Send + Copy + 'static,
{
    let now = Instant::now();
    println!("Running {} on {}", algorithm, function);

    let progress_counter = Arc::new(Mutex::new((0, Duration::new(0, 0))));

    print!("Progress: ");
    let runtimes = join_all((0..runs).map(|_i| {
        runtime.spawn(run_task(
            algorithm,
            function,
            progress_counter.clone(),
            runs,
            threads,
            launch::get_fitness_evaluations,
        ))
    }))
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

async fn run_algorithm_with_tl<A, F>(
    runtime: &Runtime,
    algorithm: A,
    function: F,
    runs: usize,
    threads: usize,
) where
    A: Algorithm + Display + Send + Copy + 'static,
    F: Function + Display + Send + Copy + 'static,
{
    let now = Instant::now();
    println!("Running {} on {}", algorithm, function);

    let progress_counter = Arc::new(Mutex::new((0, Duration::new(0, 0))));

    print!("Progress: ");
    let mut tasks = JoinSet::new();
    (0..runs).for_each(|_i| {
        tasks.spawn_on(
            run_task(
                algorithm,
                function,
                progress_counter.clone(),
                runs,
                threads,
                launch::get_fitness_evaluations_with_n3_time_limit,
            ),
            runtime.handle(),
        );
    });

    let mut runtimes = Vec::new();

    while let Some(res) = tasks.join_next().await {
        let idx = res.expect("join error");
        match idx {
            Some(x) => runtimes.push(x),
            None => {
                tasks.abort_all();
                println!();
                println!("Failed");
                println!();
                return;
            }
        }
    }

    println!();
    println!("Runtimes: {runtimes:?}");
    let average = runtimes.into_iter().sum::<usize>() as f64 / runs as f64;
    println!("Average: {average}");

    let elapsed = now.elapsed();
    println!("Evaluated in {elapsed:.2?}");
    println!()
}
