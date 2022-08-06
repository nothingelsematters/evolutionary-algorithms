use evolutionary_algorithms::experiments;

#[tokio::main]
async fn main() {
    experiments::runtime::runtime_experiment().await;
}
