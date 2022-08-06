use std::{io::Write, time::Instant};

use crate::{
    algorithm::{self, Algorithm},
    draw::save_plot,
    function::{self, Function},
};

async fn run_task(i: i32, algorithm: impl Algorithm, function: impl Function) -> usize {
    print!("{} ", i);
    std::io::stdout().flush().expect("stdout flush");
    algorithm.runtime(&function)
}

pub async fn runtime_experiment() {
    let mu_getters: Vec<Box<dyn Fn(usize) -> usize>> = vec![
        // Box::new(|_: usize| 2),
        // Box::new(|x: usize| (x as f64).log2() as usize),
        // Box::new(|x: usize| (x as f64).sqrt() as usize),
        // Box::new(|x: usize| ((x as f64) / 4.0).ceil() as usize),
    ];

    for mu_getter in mu_getters {
        let ns: Vec<usize> = (5..=13).map(|x| 1 << x).collect();

        for n in ns {
            let mu = mu_getter(n);

            let algorithm =
                algorithm::mu_plus_one::ConvexHullMaximization::new(mu, 0.99, 1.0 / (n as f64));
            let function = function::OneMax::new(n);

            let now = Instant::now();
            println!("Running {} on {}", algorithm, function);

            let mut runtimes = Vec::new();

            let tasks: Vec<_> = (0..128)
                .map(|i| tokio::task::spawn(run_task(i, algorithm, function)))
                .collect();

            for task in tasks {
                runtimes.push(task.await.expect("evaluating"));
            }

            println!();
            println!("{:?}", runtimes);

            let elapsed = now.elapsed();
            println!("Evaluated in {:.2?}\n", elapsed);
        }
    }

    let average_results = vec![
        (
            "2",
            vec![
                (5.0, 261.6640625 / 32.0),
                (6.0, 630.59375 / 64.0),
                (7.0, 1507.2109375 / 128.0),
                (8.0, 3326.6796875 / 256.0),
                (9.0, 7902.2109375 / 512.0),
                (10.0, 17636.421875 / 1024.0),
                (11.0, 39929.53125 / 2048.0),
                (12.0, 85914.2890625 / 4096.0),
                (13.0, 188340.6640625 / 8192.0),
            ],
        ),
        (
            "log2(x)",
            vec![
                (5.0, 279.3515625 / 32.0),
                (6.0, 684.6796875 / 64.0),
                (7.0, 1741.7578125 / 128.0),
                (8.0, 3837.671875 / 256.0),
                (9.0, 8663.5703125 / 512.0),
                (10.0, 19633.015625 / 1024.0),
                (11.0, 45917.953125 / 2048.0),
                (12.0, 96628.0390625 / 4096.0),
                (13.0, 210473.3984375 / 8192.0),
            ],
        ),
        (
            "sqrt(x)",
            vec![
                (5.0, 264.3203125 / 32.0),
                (6.0, 671.703125 / 64.0),
                (7.0, 1688.671875 / 128.0),
                (8.0, 4114.9375 / 256.0),
                (9.0, 9395.140625 / 512.0),
                (10.0, 22096.03125 / 1024.0),
                (11.0, 50834.9609375 / 2048.0),
                (12.0, 117448.34375 / 4096.0),
            ],
        ),
        (
            "x / 4",
            vec![
                (5.0, 262.390625 / 32.0),
                (6.0, 675.078125 / 64.0),
                (7.0, 1728.671875 / 128.0),
                (8.0, 4211.8828125 / 256.0),
                (9.0, 10256.4375 / 512.0),
                (10.0, 23351.7890625 / 1024.0),
                (11.0, 57130.4765625 / 2048.0),
            ],
        ),
    ];

    let y_min = average_results
        .iter()
        .map(|(_, vs)| vs.iter().map(|(_, x)| *x as u32).min().unwrap())
        .min()
        .unwrap();

    let y_max = average_results
        .iter()
        .map(|(_, vs)| vs.iter().map(|(_, x)| *x as u32).max().unwrap())
        .max()
        .unwrap();

    save_plot(
        "plots/runtime-one-max.png".to_owned(),
        average_results,
        5.0..13.0,
        y_min as f64..y_max as f64,
        "(μ + 1) w/ chm average runtimes: x = log2(n), y = avg / n",
        650,
        500,
    )
    .unwrap();
}
