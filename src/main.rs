use std::collections::HashMap;

use evolutionary_algorithms::algorithm::{mu_plus_one::MuPlusOne, Algorithm};
use evolutionary_algorithms::draw;
use evolutionary_algorithms::function::jump::Jump;

fn main() {
    let n = 512; // 32, 64, 128, 256, 512, 1024
    let k = 4; // 2, 4, 6

    let mu = 2; // 2, log(n), n/k
    let crossover_probability = 0.99; // 0.99 (close to 1)
    let mutation_rate = 1.0 / n as f64; // 1/n

    let algorithm = MuPlusOne::new(mu, crossover_probability, mutation_rate).unwrap();
    let function = Jump::new(n, k);

    let mut result = Vec::new();
    let mut map = HashMap::new();

    for i in 0..1 {
        println!("{}", i);
        let min = algorithm.run(Box::new(function));
        result.push(min);
        *map.entry(min).or_insert(0) += 1;
    }

    let title = format!(
        "(μ + 1), μ = {} ( n/k ), p_c = {}, p_m = {} ( 1/n ) on Jump({}, {})",
        mu, crossover_probability, mutation_rate, n, k
    );
    let text = format!("{:?}", map);
    let file_path = format!(
        "plots/plot_nk_{}_1n_Jump({},{}).html",
        crossover_probability, n, k
    );

    draw::save_box_plot(&title, &text, &file_path, result);
}
