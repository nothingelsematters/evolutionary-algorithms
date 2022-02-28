use evolutionary_algorithms::algorithm::{mu_plus_one, mu_plus_one::MuPlusOne};
use evolutionary_algorithms::draw;
use evolutionary_algorithms::function::jump::Jump;
use evolutionary_algorithms::function::Function;
use evolutionary_algorithms::MAP;

async fn run_task(i: i32, algorithm: mu_plus_one::Common, function: impl Function) {
    println!("{}", i);
    let algorithm = &algorithm as &dyn MuPlusOne;
    algorithm.run(&function)
}

#[tokio::main]
async fn main() {
    // constants
    let n = 512; // 32, 64, 128, 256, 512, 1024
    let k = 4; // 2, 4, 6

    let mu = 2; // 2, log(n), n/k
    let crossover_probability = 0.99; // 0.99 (close to 1)
    let mutation_rate = 1.0 / n as f64; // 1/n

    let runs = 10_000;

    // processing
    let algorithm = mu_plus_one::Common::new(mu, crossover_probability, mutation_rate);
    let function = Jump::new(n, k);

    let tasks: Vec<_> = (0..runs)
        .map(|i| tokio::task::spawn(run_task(i, algorithm, function)))
        .collect();
    for task in tasks {
        task.await.ok();
    }

    let guard = MAP.lock().unwrap();
    let mut vec = guard
        .iter()
        .map(|(key, (number, value))| (*key as usize, value / number))
        .collect::<Vec<_>>();
    vec.sort_unstable();

    // saving results
    println!("{:?}", vec);
    let title = format!(
        "(μ + 1) common: μ = {}, p_c = {}, p_m = {} (1/n) on Jump({}, {}), {} runs",
        mu, crossover_probability, mutation_rate, n, k, runs,
    );
    draw::save_plot(vec, &title, "1 positions for first max fitness").unwrap();
}
