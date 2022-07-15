use evolutionary_algorithms::experiments;

#[tokio::main]
async fn main() {
    experiments::dynamic_metrics::dynamic_metrics_experiment();
}
