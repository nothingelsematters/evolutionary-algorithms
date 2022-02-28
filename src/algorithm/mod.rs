use bit_vec::BitVec;
use rand::Rng;

pub mod mu_plus_one;

fn mutate(bitvec: &BitVec, mutation_rate: f64) -> BitVec {
    let mut rng = rand::thread_rng();

    bitvec
        .iter()
        .map(|x| {
            if rng.gen::<f64>() < mutation_rate {
                !x
            } else {
                x
            }
        })
        .collect()
}

fn crossover(left: &BitVec, right: &BitVec, crossover_bias: f64) -> BitVec {
    let mut rng = rand::thread_rng();

    left.iter()
        .zip(right)
        .map(|(x, y)| {
            if rng.gen::<f64>() > crossover_bias {
                x
            } else {
                y
            }
        })
        .collect()
}
