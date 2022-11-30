use crate::draw::utils::draw_runtime;

#[tokio::test]
async fn draw_rugged_one_max() {
    draw_runtime(
        "rugged-runtime/mu-plus-ones",
        "RuggedOneMax average runtimes: x = log2(n), y = avg / (nlog(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        vec![
            // (
            //     "(μ + 1), μ = 2",
            //     vec![
            //         (32.0, 2293.078125),
            //         (64.0, 7952.890625),
            //         (128.0, 34257.7890625),
            //         (256.0, 131694.2265625),
            //         (512.0, 588612.4609375),
            //         (1024.0, 2118529.8984375),
            //         (2048.0, 9085487.5234375),
            //         (4096.0, 40115516.5703125),
            //     ],
            // ),
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
        ],
    );

    draw_runtime(
        "rugged-runtime/one-plus-lambda-lambda",
        "(1 + (λ, λ)) on RuggedOneMax average runtimes: x = log2(n), y = avg / n^2",
        |(n, iters)| (n.log2(), iters / (n * n)),
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

#[tokio::test]
async fn draw_one_max() {
    draw_runtime(
        "rugged-runtime/mu-plus-ones",
        "OneMax average runtimes: x = log2(n), y = avg / (nlog(n))",
        |(n, iters)| (n.log2(), iters / (n * n.log2())),
        vec![
            // (
            //     "(μ + 1), μ = 2",
            //     vec![
            //         (32.0, 2293.078125),
            //         (64.0, 7952.890625),
            //         (128.0, 34257.7890625),
            //         (256.0, 131694.2265625),
            //         (512.0, 588612.4609375),
            //         (1024.0, 2118529.8984375),
            //         (2048.0, 9085487.5234375),
            //         (4096.0, 40115516.5703125),
            //     ],
            // ),
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
        ],
    );
}
