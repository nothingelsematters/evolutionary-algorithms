use tokio::runtime::Runtime;

mod draw;
mod optimal_parameters;
mod runtimes;

const THREADS: usize = 32;
const RUNS: usize = 128;

fn runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(THREADS)
        .build()
        .expect("thread pool building")
}
