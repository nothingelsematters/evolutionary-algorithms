use evolutionary_algorithms::experiments;

#[tokio::main]
async fn main() {
    experiments::rugged_runtime::rugged_runtime_experiment().await;
    // experiments::rugged_runtime::draw().await;
}
