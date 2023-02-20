#![allow(dead_code)]
use std::{io::Write, time::Instant};

use crate::{
    algorithm::{self, launch::get_iterations, Algorithm},
    draw::utils::draw_runtime,
    function::{self, Function},
};

#[tokio::test]
async fn runtime_experiment() {
    // run().await;
    draw().await;
}

async fn run_task<A, F>(i: i32, algorithm: A, function: F) -> usize
where
    A: Algorithm + Send,
    F: Function + Send,
{
    print!("{} ", i);
    std::io::stdout().flush().expect("stdout flush");
    get_iterations(&algorithm, &function)
}

async fn run() {
    let mu_getters: Vec<Box<dyn Fn(usize) -> usize>> = vec![
        Box::new(|_: usize| 2),
        Box::new(|x: usize| (x as f64).log2() as usize),
        Box::new(|x: usize| (x as f64).sqrt() as usize),
        Box::new(|x: usize| ((x as f64) / 4.0).ceil() as usize),
    ];

    for mu_getter in mu_getters {
        let ns: Vec<usize> = (11..=11).map(|x| 1 << x).collect();

        for n in ns {
            let mu = mu_getter(n);

            let algorithm = algorithm::mu_plus_one::Common::new(mu, 0.99, 1.0 / (n as f64));
            let function = function::LeadingOnes::new(n);

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
}

async fn draw() {
    draw_runtime(
        "runtime/common-one-max",
        "",
        "",
        "(μ + 1) on OneMax average runtimes: x = log2(n), y = avg / (nlog(n))",
        |x| x,
        vec![
            (
                "2",
                vec![
                    (5.0, 1.348974609375),
                    (6.0, 1.41668701171875),
                    (7.0, 1.4236624581473214),
                    (8.0, 1.4436759948730469),
                    (9.0, 1.4407263861762152),
                    (10.0, 1.4343467712402345),
                    (11.0, 1.4191755814985796),
                    (12.0, 1.4516490300496419),
                    (13.0, 1.4494585624107947),
                ],
            ),
            (
                "log2(x)",
                vec![
                    (5.0, 1.3314453125),
                    (6.0, 1.3318888346354167),
                    (7.0, 1.3749302455357142),
                    (8.0, 1.2973365783691406),
                    (9.0, 1.3248867458767362),
                    (10.0, 1.3326820373535155),
                    (11.0, 1.3573702031915837),
                    (12.0, 1.3666160901387532),
                    (13.0, 1.3410759705763597),
                ],
            ),
            (
                "sqrt(x)",
                vec![
                    (5.0, 1.309326171875),
                    (6.0, 1.33203125),
                    (7.0, 1.3301304408482142),
                    (8.0, 1.434173583984375),
                    (9.0, 1.5009663899739583),
                    (10.0, 1.6014823913574219),
                    (11.0, 1.7041730013760654),
                    (12.0, 1.8595093091328938),
                    (13.0, 1.9724070475651667),
                ],
            ),
            (
                "x / 4",
                vec![
                    (5.0, 1.3814453125),
                    (6.0, 1.5381062825520833),
                    (7.0, 1.6531546456473214),
                    (8.0, 1.8591384887695312),
                    (9.0, 2.034823947482639),
                    (10.0, 2.2254104614257812),
                    (11.0, 2.4905392039905894),
                    (12.0, 2.979024092356364),
                    (13.0, 3.794988118685209),
                ],
            ),
        ],
    );

    draw_runtime(
        "runtime/common-leading-ones",
        "",
        "",
        "(μ + 1) on LeadingOnes average runtimes: x = log2(n), y = avg / n^2",
        |x| x,
        vec![
            (
                "2",
                vec![
                    (5.0, 0.8868026733398438),
                    (6.0, 0.9063186645507812),
                    (7.0, 0.8625020980834961),
                    (8.0, 0.8670284748077393),
                    (9.0, 0.860436886548996),
                    (10.0, 0.8530555069446564),
                ],
            ),
            (
                "log2(x)",
                vec![
                    (5.0, 0.8976974487304688),
                    (6.0, 0.8642654418945312),
                    (7.0, 0.8931021690368652),
                    (8.0, 0.870549201965332),
                    (9.0, 0.8620652556419373),
                    (10.0, 0.8598647266626358),
                ],
            ),
            (
                "sqrt(x)",
                vec![
                    (5.0, 0.8444061279296875),
                    (6.0, 0.896759033203125),
                    (7.0, 0.8845338821411133),
                    (8.0, 0.8794373273849487),
                    (9.0, 0.8710023760795593),
                    (10.0, 0.8775895535945892),
                ],
            ),
            (
                "x / 4",
                vec![
                    (5.0, 0.9615707397460938),
                    (6.0, 0.9959278106689453),
                    (7.0, 1.0107364654541016),
                    (8.0, 1.0594673156738281),
                    (9.0, 1.1039375364780426),
                    (10.0, 1.1222889700601266),
                ],
            ),
        ],
    );
}
