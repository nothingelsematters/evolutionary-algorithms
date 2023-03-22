use std::fmt::Display;
use tokio::runtime::Runtime;

use super::{runtime, RUNS, THREADS};
use crate::{
    algorithm::{self, Algorithm},
    function::{self, Function},
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
    let population_sizes: Vec<usize> = [10, 6].into_iter().map(|x| 1 << x).collect();
    // let population_sizes: Vec<usize> = (5..log_n as u32).map(|x| 1 << x).collect();

    let function = function::RuggedOneMax::new(2, n);

    for population_size in population_sizes {
        // let mu_common =
        //     algorithm::mu_plus_one::Common::new(population_size, 0.99, 1.0 / (n as f64));
        // run_algorithm(&runtime, mu_common, function).await;

        let one_plus_lambda_lambda = algorithm::OnePlusLambdaLambda::new(
            population_size,
            population_size as f64 / n as f64,
            1.0 / population_size as f64,
        );

        run_algorithm(&runtime, one_plus_lambda_lambda, function).await;
    }
    println!();
}
