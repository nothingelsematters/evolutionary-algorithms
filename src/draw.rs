use plotly::box_plot::BoxMean;
use plotly::common::{Orientation, Title};
use plotly::layout::{Axis, BoxMode};
use plotly::{BoxPlot, Layout, Plot};

use plotters::prelude::*;

pub fn save_plot(
    result: Vec<(usize, usize)>,
    caption: &str,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plots/plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            result.iter().map(|(x, _)| x).min().unwrap() - 1
                ..result.iter().map(|(x, _)| x).max().unwrap() + 1,
            result.iter().map(|(_, y)| y).min().unwrap() - 1
                ..result.iter().map(|(_, y)| y).max().unwrap() + 1,
        )?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(result.iter().cloned(), &RED))?
        .label(label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            result.into_iter().map(|(x, _)| (x, x)),
            &BLACK,
        ))?
        .label("fitness")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn save_box_plot(title: &str, text: &str, file_path: &str, nums: Vec<usize>) {
    let len = nums.len();
    let trace = BoxPlot::new_xy(nums, vec![0; len])
        .orientation(Orientation::Horizontal)
        .box_mean(BoxMean::True);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title(Title::new(title))
        .x_axis(Axis::new().title(Title::new(text)).zero_line(false))
        .box_mode(BoxMode::Group);

    plot.set_layout(layout);
    plot.to_html(file_path)
}
