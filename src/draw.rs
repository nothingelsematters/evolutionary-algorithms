use image::codecs::gif::GifEncoder;
use image::{Frame, ImageBuffer};
use plotters::backend::BGRXPixel;
use plotters::prelude::*;
use std::fs::File;
use std::ops::Range;
use std::path::Path;

pub fn get_plot(
    result: Vec<(usize, usize)>,
    x: Range<usize>,
    y: Range<usize>,
    caption: &str,
    label: &str,
    width: usize,
    height: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut data = vec![0u8; width * height * 4];

    {
        let root = BitMapBackend::<'_, BGRXPixel>::with_buffer_and_format(
            &mut data,
            (width as u32, height as u32),
        )
        .expect("")
        .into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("monospace", 15).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x, y)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(result.into_iter(), &RED))?
            .label(label)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
    }

    Ok(data)
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
