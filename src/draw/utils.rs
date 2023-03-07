use super::save_plot_with_description;

pub fn draw_runtime<F>(
    file_name: &str,
    x_description: &'static str,
    y_description: &'static str,
    caption: &str,
    transformation: F,
    results: Vec<(&str, Vec<(f64, f64)>)>,
) where
    F: Copy + Fn((f64, f64)) -> (f64, f64),
{
    let results = results
        .into_iter()
        .map(|(i, x)| (i, x.into_iter().map(transformation).collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let _y_min = results
        .iter()
        .flat_map(|(_, vs)| vs.iter().map(|(_, y)| *y))
        .min_by_key(|y| *y as u64)
        .unwrap();

    let y_max = results
        .iter()
        .flat_map(|(_, vs)| vs.iter().map(|(_, y)| *y))
        .max_by_key(|y| *y as u64)
        .unwrap();

    let x_min = results
        .iter()
        .flat_map(|(_, vs)| vs.iter().map(|(x, _)| *x))
        .min_by_key(|y| *y as u64)
        .unwrap();

    let x_max = results
        .iter()
        .flat_map(|(_, vs)| vs.iter().map(|(x, _)| *x))
        .max_by_key(|y| *y as u64)
        .unwrap();

    save_plot_with_description(
        format!("results/{}.png", file_name),
        results,
        x_min..x_max,
        0.0..y_max,
        x_description,
        y_description,
        caption,
        650,
        500,
    )
    .unwrap();
}
