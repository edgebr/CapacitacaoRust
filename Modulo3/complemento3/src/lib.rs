#![allow(unused)]

mod physical;
mod signals;

use crate::signals::sine::Sine;
use crate::signals::square::SquareWave;
use crate::signals::SawTooth;
use signals::*;
use std::io::Error as Err;

#[test]
fn test_graphs() {
    let total = 1_000;
    let x_max = 2.0 * std::f32::consts::PI;

    let sin = Sine::new(x_max, 1.0, 0.0);
    let saw_tooth = SawTooth::new(x_max, 1.0, 0.0);
    let square = SquareWave::new(x_max, 1.0, 0.0);

    assert!(plot_serie(
        (0..total)
            .map(|t| {
                let t = t as f32 * (x_max / (total as f32 - 1.0));
                // 0.0
                sin.compute(t)
            })
            .collect(),
        "sin",
        total as f32,
        (-1.0, 1.0),
        x_max,
        "sin.png",
    )
    .is_ok());
    assert!(plot_serie(
        (0..total)
            .map(|t| {
                let t = t as f32 * (x_max / (total as f32 - 1.0));
                // 0.0
                saw_tooth.compute(t)
            })
            .collect(),
        "sawtooth",
        total as f32,
        (0.0, 1.1),
        x_max,
        "sawtooth.png",
    )
    .is_ok());
    assert!(plot_serie(
        (0..total)
            .map(|t| {
                let t = t as f32 * (x_max / (total as f32 - 1.0));
                // 0.0
                square.compute(t)
            })
            .collect(),
        "square",
        total as f32,
        (0.0, 1.1),
        x_max,
        "square.png",
    )
    .is_ok());
}

#[test]
fn test_system() {
    let total = 1_000;
    let x_max = 1.5;

    let mut output = vec![];
    let mut system = physical::System::default();

    for t in 0..total {
        let t = t as f32 * (x_max / (total as f32 - 1.0));
        output.push(system.sensor(t));
    }

    assert!(plot_serie(
        output,
        "system",
        total as f32,
        (0.0, 1.5),
        x_max,
        "system.png",
    )
    .is_ok());
}

fn plot_serie(
    serie: Vec<f32>,
    caption: &str,
    total: f32,
    y_lim: (f32, f32),
    x_max: f32,
    file_output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;

    let root = BitMapBackend::new(file_output, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..x_max, y_lim.0..y_lim.1)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            serie
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f32 * (x_max / (total - 1.0)), *y)),
            &RED,
        ))?
        .label(caption)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
