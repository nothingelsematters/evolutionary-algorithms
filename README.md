# Experiments

## Functions

- [LeadingOnes](src/function/leading_ones.rs)
- [OneMax](src/function/one_max.rs)
- [RuggedOneMax](src/function/rugged_one_max.rs)
- [Jump](src/function/jump.rs)

## Algorithms

- [(μ + 1)](src/algorithm/mu_plus_one/common.rs)
- [(μ + 1) with convex hull maximization](src/algorithm/mu_plus_one/convex_hull_maximization.rs)
- [(1 + (λ, λ))](src/algorithm/one_plus_lambda_lambda.rs)

# Slurm evaluations cheat sheet

- task.sh:

  ```sh
  #!/bin/bash

  cargo test --release -- --nocapture rugged_optimal_parameters
  ```

- run: `sbatch --cpus-per-task=32 --mem=4G --time=14-00:00:00 task.sh`

- check: `squeue -o "%.18i %.9P %.8j %.8C %.8u %.8T %.10m %.10M %.9l %.6D %R" | grep <username>`

- output: `cat slurm-<task_id>.out`

- cancel: `scancel <task_id>`
