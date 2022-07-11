use evolutionary_algorithms::{
    algorithm::{mu_plus_one, Algorithm},
    draw,
    // draw,
    function::{Function, Jump},
    COUNT,
    DRIFT,
};

#[tokio::main]
async fn main() {
    async fn run_task(i: i32, algorithm: impl Algorithm, function: impl Function) {
        println!("{}", i);
        algorithm.run(function)
    }

    // constants
    let n = 512; // 32, 64, 128, 256, 512, 1024
    let k = 4; // 2, 4, 6

    let mu = (n as f64).log2() as usize; // 2, (n as f64).log2() as usize, n/k
    let crossover_probability = 0.99; // 0.99 (close to 1)
    let mutation_rate = 1.0 / n as f64; // 1/n

    let runs = 1000;

    // processing
    let algorithm = mu_plus_one::Common::new(mu, crossover_probability, mutation_rate);
    let function = Jump::new(n, k);

    for i in 0..runs {
        run_task(i, algorithm, function).await;
    }

    // let tasks: Vec<_> = (0..runs)
    //     .map(|i| tokio::task::spawn(run_task(i, algorithm, function)))
    //     .collect();
    // for task in tasks {
    //     task.await.ok();
    // }

    let guard = COUNT.lock().unwrap();
    let mut vec = guard
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let a = (index, (*value as i32 / runs) as usize);
            println!("{:?}", a);
            a
        })
        .collect::<Vec<_>>();
    vec.sort_unstable();

    // // saving results
    let title = format!(
        "(μ + 1) common: μ = {} (log(n)), p_c = {}, p_m = {} (1/n) on Jump({}, {}), {} runs",
        mu, crossover_probability, mutation_rate, n, k, runs,
    );
    println!("{}", title);
    draw::save_plot(vec, &title, "counts for fitness").unwrap();

    // let count = COUNT.lock().unwrap();
    // let drift = DRIFT.lock().unwrap();
    // for i in 0..mu {
    //     println!(
    //         "{}: count[i] = {:4}, drift[i] = {}, drift[i] / count[i] = {}",
    //         i,
    //         count[i],
    //         drift[i],
    //         drift[i] as f64 / count[i] as f64,
    //     );
    // }
}
