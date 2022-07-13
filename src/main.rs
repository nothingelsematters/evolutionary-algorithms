use evolutionary_algorithms::experiments;

#[tokio::main]
async fn main() {
    experiments::dynamic_fitness::dynamic_fitness_experiment();
}
