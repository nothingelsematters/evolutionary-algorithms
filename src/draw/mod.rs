use image::codecs::gif::GifEncoder;
use image::{Frame, ImageBuffer};
use plotters::backend::BGRXPixel;
use plotters::coord::Shift;
use plotters::prelude::*;
use std::fs::File;
use std::ops::Range;
use std::path::Path;

pub mod utils;

fn draw_plot_with_description<DB>(
    root: DrawingArea<DB, Shift>,
    results: Vec<(&str, Vec<(f64, f64)>)>,
    x: Range<f64>,
    y: Range<f64>,
    x_description: &'static str,
    y_description: &'static str,
    caption: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    DB: DrawingBackend,
    DB::ErrorType: 'static,
{
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("monospace", 15).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x, y)?;

    chart
        .configure_mesh()
        .x_desc(x_description)
        .y_desc(y_description)
        .draw()?;

    let colors = vec![RED, BLUE, GREEN, YELLOW, CYAN, MAGENTA, WHITE, BLACK];

    for ((label, result), color) in results.into_iter().zip(colors.into_iter()) {
        chart
            .draw_series(LineSeries::new(result.into_iter(), color))?
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

fn draw_plot<DB>(
    root: DrawingArea<DB, Shift>,
    results: Vec<(&str, Vec<(f64, f64)>)>,
    x: Range<f64>,
    y: Range<f64>,
    caption: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    DB: DrawingBackend,
    DB::ErrorType: 'static,
{
    draw_plot_with_description(root, results, x, y, "", "", caption)
}

pub fn get_plot(
    results: Vec<(&str, Vec<(f64, f64)>)>,
    x: Range<f64>,
    y: Range<f64>,
    caption: &str,
    width: usize,
    height: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut data = vec![0u8; width * height * 4];

    {
        let backend = BitMapBackend::<'_, BGRXPixel>::with_buffer_and_format(
            &mut data,
            (width as u32, height as u32),
        )
        .expect("backend creation")
        .into_drawing_area();
        draw_plot(backend, results, x, y, caption)?;
    }

    Ok(data)
}

#[allow(clippy::too_many_arguments)]
pub fn save_plot_with_description(
    file_name: String,
    results: Vec<(&str, Vec<(f64, f64)>)>,
    x: Range<f64>,
    y: Range<f64>,
    x_description: &'static str,
    y_description: &'static str,
    caption: &str,
    width: usize,
    height: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new(&file_name, (width as u32, height as u32)).into_drawing_area();
    draw_plot_with_description(
        backend,
        results,
        x,
        y,
        x_description,
        y_description,
        caption,
    )
}

pub fn save_plot(
    file_name: String,
    results: Vec<(&str, Vec<(f64, f64)>)>,
    x: Range<f64>,
    y: Range<f64>,
    caption: &str,
    width: usize,
    height: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new(&file_name, (width as u32, height as u32)).into_drawing_area();
    draw_plot(backend, results, x, y, caption)
}

pub fn save_gif<P, I>(file_path: P, width: usize, height: usize, speed: i32, frame_iter: I)
where
    P: AsRef<Path>,
    I: Iterator<Item = Vec<u8>>,
{
    let gif_file = File::create(&file_path).expect("animation file creation");
    let mut gif_encoder = GifEncoder::new_with_speed(gif_file, speed);

    for (i, frame) in frame_iter.enumerate() {
        println!("Adding frame #{}", i);
        let plot =
            ImageBuffer::from_raw(width as u32, height as u32, frame).expect("plot decoding");

        gif_encoder
            .encode_frame(Frame::new(plot))
            .expect("plot into animation encoding");
    }
}
