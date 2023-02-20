use std::fmt::Display;
use tokio::runtime::Runtime;

use super::{runtime, RUNS, THREADS};
use crate::{
    algorithm::{self, Algorithm},
    function::{self, Function},
};

#[tokio::test]
async fn rugged_runtime() {
    async fn run_algorithm<A, F>(runtime: &Runtime, algorithm: A, function: F) -> f64
    where
        A: Algorithm + Display + Send + Copy + 'static,
        F: Function + Display + Send + Copy + 'static,
    {
        crate::experiment::run_algorithm(runtime, algorithm, function, RUNS, THREADS).await
    }

    let runtime = runtime();

    let ns: Vec<usize> = (20..=20).map(|x| 1 << x).collect();

    #[allow(clippy::type_complexity)]
    let mu_getters: Vec<(&str, Box<dyn Fn(usize) -> usize>)> = vec![
        ("2", Box::new(|_: usize| 2)),
        ("log2(n)", Box::new(|x: usize| (x as f64).log2() as usize)),
        ("sqrt(n)", Box::new(|x: usize| (x as f64).sqrt() as usize)),
    ];

    #[allow(clippy::type_complexity)]
    let lambda_getters: Vec<(&str, Box<dyn Fn(usize) -> usize>)> = vec![
        (
            "sqrt(log2(n))",
            Box::new(|x: usize| (x as f64).log2().sqrt() as usize),
        ),
        ("log2(n)", Box::new(|x: usize| (x as f64).log2() as usize)),
        ("sqrt(n)", Box::new(|x: usize| (x as f64).sqrt() as usize)),
    ];

    for n in ns {
        let function = function::RuggedOneMax::new(2, n);

        for (mu_getter_name, mu_getter) in mu_getters.iter() {
            let mu = mu_getter(n);

            let mu_common = algorithm::mu_plus_one::Common::new(mu, 0.99, 1.0 / (n as f64));
            println!("{mu_getter_name}");
            run_algorithm(&runtime, mu_common, function).await;

            let mu_chm =
                algorithm::mu_plus_one::ConvexHullMaximization::new(mu, 0.99, 1.0 / (n as f64));
            println!("{mu_getter_name}");
            run_algorithm(&runtime, mu_chm, function).await;
        }
        println!();

        for (lambda_getter_name, lambda_getter) in lambda_getters.iter() {
            let lambda = lambda_getter(n);
            let one_plus_lambda_lambda = algorithm::OnePlusLambdaLambda::new(
                lambda,
                lambda as f64 / function.n() as f64,
                1.0 / lambda as f64,
            );

            println!("{lambda_getter_name}");
            run_algorithm(&runtime, one_plus_lambda_lambda, function).await;
        }
        println!();
    }
}
