use crate::draw::utils::draw_runtime;

#[test]
fn draw_rugged_2() {
    let mpoga = vec![
        (
            "(μ + 1), μ = 2",
            vec![
                (32.0, 2293.078125),
                (64.0, 7952.890625),
                (128.0, 34257.7890625),
                (256.0, 131694.2265625),
                (512.0, 588612.4609375),
                (1024.0, 2118529.8984375),
                (2048.0, 9085487.5234375),
                (4096.0, 40115516.5703125),
            ],
        ),
        (
            "(μ + 1), μ = log2(n)",
            vec![
                (32.0, 824.9609375),
                (64.0, 2534.3046875),
                (128.0, 5935.859375),
                (256.0, 15509.21875),
                (512.0, 49459.1640625),
                (1024.0, 119370.78125),
                (2048.0, 364061.2734375),
                (4096.0, 1194703.8828125),
            ],
        ),
        (
            "(μ + 1), μ = sqrt(n)",
            vec![
                (32.0, 814.359375),
                (64.0, 1690.359375),
                (128.0, 3725.140625),
                (256.0, 7510.703125),
                (512.0, 17795.3359375),
                (1024.0, 39292.4609375),
                (2048.0, 99250.8515625),
                (4096.0, 229238.09375),
                (8192.0, 586752.984375),
            ],
        ),
        (
            "(μ + 1) w/ chm, μ = 2",
            vec![
                (32.0, 388.8828125),
                (64.0, 934.8203125),
                (128.0, 2020.140625),
                (256.0, 4693.34375),
                (512.0, 10618.609375),
                (1024.0, 22663.921875),
                (2048.0, 48953.5703125),
                (4096.0, 104520.703125),
                (8192.0, 232147.78125),
                (16384.0, 478313.6484375),
            ],
        ),
        (
            "(μ + 1) w/ chm, μ = log2(n)",
            vec![
                (32.0, 328.953125),
                (64.0, 751.46875),
                (128.0, 1731.25),
                (256.0, 4066.25),
                (512.0, 8675.203125),
                (1024.0, 19369.421875),
                (2048.0, 43506.2578125),
                (4096.0, 94715.9140625),
                (8192.0, 202706.0),
            ],
        ),
        (
            "(μ + 1) w/ chm, μ = sqrt(n)",
            vec![
                (32.0, 336.21875),
                (64.0, 745.390625),
                (128.0, 1740.703125),
                (256.0, 4000.3671875),
                (512.0, 9312.078125),
                (1024.0, 21066.0625),
                (2048.0, 48739.6953125),
                (4096.0, 111678.75),
            ],
        ),
    ];

    draw_runtime(
        "rugged-onemax/mu-plus-ones",
        "average / (n log2(n))",
        "log2(n)",
        "RuggedOneMax average runtimes: x = log2(n), y = avg / (nlog(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        mpoga.clone(),
    );

    draw_runtime(
        "average / (n log2(n))ed-runtime/mu-plus-one",
        "",
        "log2(n)",
        "RuggedOneMax average runtimes: x = log2(n), y = avg / (nlog(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        mpoga[..3].to_vec(),
    );

    draw_runtime(
        "rugged-onemax/mu-plus-one-chm",
        "average / (n log2(n))",
        "log2(n)",
        "RuggedOneMax average runtimes: x = log2(n), y = avg / (nlog(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        mpoga[3..].to_vec(),
    );

    draw_runtime(
        "rugged-onemax/one-plus-lambda-lambda",
        "",
        "",
        "(1 + (λ, λ)) on RuggedOneMax average runtimes: x = log2(n), y = avg / (n^(3/2))",
        |(n, iters)| (n.log2(), iters / (n.powf(3.0 / 2.0))),
        vec![
            (
                "sqrt(log2(n))",
                vec![
                    (32.0, 3385.6875),
                    (64.0, 15116.0312),
                    (128.0, 64343.187),
                    (256.0, 217575.75),
                    (512.0, 655487.9062),
                ],
            ),
            (
                "log2(n)",
                vec![
                    (32.0, 1736.484375),
                    (64.0, 5467.6875),
                    (128.0, 19970.015),
                    (256.0, 67064.25),
                    (512.0, 227248.734),
                ],
            ),
            (
                "sqrt(n)",
                vec![
                    (32.0, 1691.015625),
                    (64.0, 4797.625),
                    (128.0, 12597.062),
                    (256.0, 41027.5),
                    (512.0, 108165.75),
                ],
            ),
        ],
    );
}

#[test]
fn draw_rugged_3_mu1() {
    let results = vec![
        (
            "(μ + 1), μ = 2",
            vec![
                (32.0, 5895.8125),
                (64.0, 64776.0),
                (128.0, 374912.0859375),
                (256.0, 3246850.3828125),
            ],
        ),
        (
            "(μ + 1), μ = log2(n)",
            vec![
                (32.0, 2122.484375),
                (64.0, 6354.59375),
                (128.0, 19430.796875),
                (256.0, 78007.7109375),
                (512.0, 282204.6328125),
                (1024.0, 1106023.5625),
            ],
        ),
        (
            "(μ + 1), μ = sqrt(n)",
            vec![
                (32.0, 2146.078125),
                (64.0, 3727.390625),
                (128.0, 5321.0859375),
                (256.0, 8586.4296875),
                (512.0, 17825.5078125),
                (1024.0, 37625.2734375),
                (2048.0, 87598.3046875),
                (4096.0, 210856.359375),
                (8192.0, 519832.03125),
                (16384.0, 1220096.4296875),
                (32768.0, 2934197.6015625),
                (65536.0, 7216446.359375),
            ],
        ),
        (
            "(μ + 1) w/ chm, μ = 2",
            vec![
                (32.0, 778.3671875),
                (64.0, 1592.1796875),
                (128.0, 3082.8515625),
                (256.0, 6807.0078125),
                (512.0, 14565.4921875),
                (1024.0, 31678.59375),
                (2048.0, 67976.921875),
                (4096.0, 142516.9765625),
                (8192.0, 306871.015625),
            ],
        ),
        (
            "(μ + 1) w/ chm, μ = log2(n)",
            vec![
                (32.0, 608.4375),
                (64.0, 1274.109375),
                (128.0, 2462.9921875),
                (256.0, 5468.4609375),
                (512.0, 11454.625),
                (1024.0, 25282.7578125),
                (2048.0, 55231.0),
                (4096.0, 118990.5078125),
                (8192.0, 256355.5234375),
            ],
        ),
        (
            "(μ + 1) w/ chm, μ = sqrt(n)",
            vec![
                (32.0, 637.5625),
                (64.0, 1189.421875),
                (128.0, 2569.9453125),
                (256.0, 5595.328125),
                (512.0, 12423.3515625),
                (1024.0, 27481.6171875),
                (2048.0, 61316.703125),
            ],
        ),
    ];

    draw_runtime(
        "rugged-onemax/rugged-one-max-3/mu-plus-one/full",
        "",
        "",
        "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / n^2",
        |(n, iters)| (n.log2(), iters / n.powi(2)),
        results.clone(),
    );

    draw_runtime(
        "rugged-onemax/rugged-one-max-3/mu-plus-one/without-common-2",
        "",
        "",
        "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / (n log2(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        results[1..].to_vec(),
    );

    // draw_runtime(
    //     "rugged-onemax/rugged-one-max-3/mu-plus-one/common-2",
    //     "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / (n^2 sqrt(n))",
    //     |(n, iters)| (n.log2(), iters / (n.powi(2) * n.sqrt())),
    //     results[0..=0].to_vec(),
    // );

    draw_runtime(
        "rugged-onemax/rugged-one-max-3/mu-plus-one/without-common-2-log",
        "",
        "",
        "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / (n log2(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        results[2..].to_vec(),
    );

    draw_runtime(
        "rugged-onemax/rugged-one-max-3/mu-plus-one/common-sqrt",
        "",
        "",
        "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / (n log2(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        results[2..=2].to_vec(),
    );

    // draw_runtime(
    //     "rugged-onemax/rugged-one-max-3/mu-plus-one/common-log-n3",
    //     "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / n^3",
    //     |(n, iters)| (n.log2(), iters / (n.powf(3.0))),
    //     results[1..=1].to_vec(),
    // );

    // draw_runtime(
    //     "rugged-onemax/rugged-one-max-3/mu-plus-one/chm-n2",
    //     "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / n^2",
    //     |(n, iters)| (n.log2(), iters / n.powi(2)),
    //     results[3..].to_vec(),
    // );

    draw_runtime(
        "rugged-onemax/rugged-one-max-3/mu-plus-one/chm-nlog",
        "",
        "",
        "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / (n log2(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        results[3..].to_vec(),
    );
}

#[test]
fn draw_rugged_3_1ll() {
    let results = vec![
        (
            "(1 + (λ, λ)) sqrt(log2(n))",
            vec![
                (32.0, 14381.96875),
                (64.0, 112233.375),
                (128.0, 925660.71875),
                (256.0, 7398737.9375),
                (512.0, 33784227.609375),
            ],
        ),
        (
            "(1 + (λ, λ)) log2(n)",
            vec![
                (32.0, 6421.40625),
                (64.0, 34264.40625),
                (128.0, 186671.40625),
                (256.0, 1092839.375),
                (512.0, 5453118.5625),
            ],
        ),
        (
            "(1 + (λ, λ)) sqrt(n)",
            vec![
                (32.0, 6651.640625),
                (64.0, 29879.375),
                (128.0, 112464.171875),
                (256.0, 528463.0),
                (512.0, 2327369.34375),
            ],
        ),
    ];

    draw_runtime(
        "rugged-onemax/rugged-one-max-3/one-plus-lambda-lambda/full",
        "",
        "",
        "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / n^2",
        |(n, iters)| (n.log2(), iters / n.powi(2)),
        results.clone(),
    );

    draw_runtime(
        "rugged-onemax/rugged-one-max-3/one-plus-lambda-lambda/without-sqrt-log",
        "",
        "",
        "RuggedOneMax(k = 3) average runtimes: x = log2(n), y = avg / n^2",
        |(n, iters)| (n.log2(), iters / n.powi(2)),
        results[1..].to_vec(),
    );
}

#[test]
fn draw_simple() {
    draw_runtime(
        "rugged-onemax/other/mu-plus-ones-simple-one-max",
        "",
        "",
        "OneMax average runtimes: x = log2(n), y = avg / (nlog(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        vec![
            (
                "(μ + 1), μ = 2",
                vec![
                    (32.0, 230.59375),
                    (64.0, 526.921875),
                    (128.0, 1329.8046875),
                    (256.0, 3003.0859375),
                    (512.0, 6698.734375),
                    (1024.0, 14792.546875),
                    (2048.0, 31708.40625),
                    (4096.0, 71308.390625),
                    (8192.0, 157091.0546875),
                ],
            ),
            (
                "(μ + 1), μ = log2(n)",
                vec![
                    (32.0, 204.2421875),
                    (64.0, 529.609375),
                    (128.0, 1163.984375),
                    (256.0, 2720.9921875),
                    (512.0, 6057.8828125),
                    (1024.0, 14149.609375),
                    (2048.0, 30111.734375),
                    (4096.0, 66659.8828125),
                    (8192.0, 146174.9921875),
                ],
            ),
            (
                "(μ + 1), μ = sqrt(n)",
                vec![
                    (32.0, 208.890625),
                    (64.0, 512.125),
                    (128.0, 1284.7734375),
                    (256.0, 2995.4609375),
                    (512.0, 6987.0625),
                    (1024.0, 16735.375),
                    (2048.0, 39089.5625),
                    (4096.0, 90566.6875),
                    (8192.0, 211086.109375),
                ],
            ),
            (
                "(μ + 1) w/ chm, μ = 2",
                vec![
                    (32.0, 178.328125),
                    (64.0, 398.9375),
                    (128.0, 894.484375),
                    (256.0, 2149.3671875),
                    (512.0, 4668.1015625),
                    (1024.0, 10537.8984375),
                    (2048.0, 23020.0078125),
                    (4096.0, 49600.9765625),
                    (8192.0, 105389.6640625),
                ],
            ),
            (
                "(μ + 1) w/ chm, μ = log2(n)",
                vec![
                    (32.0, 190.7578125),
                    (64.0, 427.4140625),
                    (128.0, 1024.7890625),
                    (256.0, 2358.5546875),
                    (512.0, 5314.6796875),
                    (1024.0, 11877.28125),
                    (2048.0, 26388.2421875),
                    (4096.0, 57424.5859375),
                    (8192.0, 125195.0),
                ],
            ),
            (
                "(μ + 1) w/ chm, μ = sqrt(n)",
                vec![
                    (32.0, 187.203125),
                    (64.0, 458.546875),
                    (128.0, 1121.140625),
                    (256.0, 2621.8125),
                    (512.0, 6217.640625),
                    (1024.0, 14765.0703125),
                    (2048.0, 35468.3984375),
                    (4096.0, 82044.15625),
                    (8192.0, 190196.7109375),
                ],
            ),
        ],
    );
}

#[test]
fn draw_rugged_optimal_10() {
    let results = vec![
        (
            "(μ + 1)",
            vec![
                (2.0, 2250282.421875),
                (4.0, 816314.640625),
                (8.0, 180495.0546875),
                (16.0, 62315.5703125),
                (32.0, 39846.0),
                (64.0, 37322.703125),
                (128.0, 34879.46875),
                (256.0, 27748.3359375),
                (512.0, 37650.046875),
            ],
        ),
        (
            "(1 + (λ, λ))",
            vec![
                (2.0, 5471479.25),
                (4.0, 2531275.5),
                (8.0, 1131533.125),
                (16.0, 567447.0),
                (32.0, 311715.0),
                (64.0, 263247.0),
                (128.0, 331414.0),
                (256.0, 700760.0),
                (512.0, 2128744.0),
            ],
        ),
    ];

    draw_runtime(
        "rugged-onemax/optimal-parameters/10-mpoga",
        "log2(μ)",
        "fitness evaluations / 10^4",
        "RuggedOneMax(k = 2) average runtimes",
        |(n, iters)| (n.log2(), iters / 10_000.0),
        results[..1].to_vec(),
    );

    draw_runtime(
        "rugged-onemax/optimal-parameters/10-mpoga-zoom",
        "log2(μ)",
        "fitness evaluations / 10^4",
        "RuggedOneMax(k = 2) average runtimes",
        |(n, iters)| (n.log2(), iters / 10_000.0),
        vec![(results[0].0, results[0].1[5..].to_vec())],
    );

    draw_runtime(
        "rugged-onemax/optimal-parameters/10-ollga",
        "log2(λ)",
        "fitness evaluations / 10^4",
        "RuggedOneMax(k = 2) average runtimes",
        |(n, iters)| (n.log2(), iters / 10_000.0),
        results[1..].to_vec(),
    );

    draw_runtime(
        "rugged-onemax/optimal-parameters/10-ollga-zoom",
        "log2(λ)",
        "fitness evaluations / 10^4",
        "RuggedOneMax(k = 2) average runtimes",
        |(n, iters)| (n.log2(), iters / 10_000.0),
        vec![(results[1].0, results[1].1[3..8].to_vec())],
    );
}

#[test]
fn draw_rugged_optimal_15() {
    let results = vec![
        (
            "(μ + 1)",
            vec![
                (16.0, 30619748.34375),
                (32.0, 4404176.7578125),
                (64.0, 2849158.015625),
                (128.0, 3296180.109375),
                (256.0, 3699597.296875),
                (512.0, 3840366.1640625),
                (1024.0, 3608969.59375),
                (2048.0, 2825939.6015625),
                (4096.0, 1617365.96875),
                (8192.0, 3159271.8046875),
                (16384.0, 6255860.5859375),
            ],
        ),
        (
            "(1 + (λ, λ))",
            vec![(128.0, 67932422.0), (256.0, 43034264.0), (512.0, 44656008.0)],
        ),
    ];

    draw_runtime(
        "rugged-onemax/optimal-parameters/15-mpoga",
        "log2(μ)",
        "fitness evaluations / 10^6",
        "RuggedOneMax(k = 2) average runtimes",
        |(n, iters)| (n.log2(), iters / 1_000_000.0),
        results[..1].to_vec(),
    );

    draw_runtime(
        "rugged-onemax/optimal-parameters/15-mpoga-zoom",
        "log2(μ)",
        "fitness evaluations / 10^6",
        "RuggedOneMax(k = 2) average runtimes",
        |(n, iters)| (n.log2(), iters / 1_000_000.0),
        vec![(results[0].0, results[0].1[1..].to_vec())],
    );

    draw_runtime(
        "rugged-onemax/optimal-parameters/15-ollga",
        "log2(λ)",
        "fitness evaluations / 10^7",
        "RuggedOneMax average runtimes",
        |(n, iters)| (n.log2(), iters / 10_000_000.0),
        results[1..].to_vec(),
    );

    // draw_runtime(
    //     "rugged-onemax/optimal-parameters/15-ollga-zoom",
    //     "log2(λ)",
    //     "fitness evaluations / 10^4",
    //     "RuggedOneMax average runtimes",
    //     |(n, iters)| (n.log2(), iters / 10_000.0),
    //     vec![(results[1].0, results[1].1[3..8].to_vec())],
    // );
}
