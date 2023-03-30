#![allow(unused)]

mod physical {
    struct System {}

    impl Default for System {
        fn default() -> Self {
            Self {}
        }
    }

    impl System {
        fn sensor(&mut self, t: f32) -> f32 {
            const TAU: f32 = 0.1;

            1.0 - f32::exp(-t / TAU)
        }
    }
}

mod signals {
    trait Wave {
        fn compute(&self, t: f32) -> f32;
    }

    struct Sine {
        period: f32,
        amplitude: f32,
        offset: f32,
    }

    impl Sine {
        fn new(period: f32, amplitude: f32, offset: f32) -> Self {
            Self {
                period,
                amplitude,
                offset,
            }
        }
    }

    impl Wave for Sine {
        fn compute(&self, t: f32) -> f32 {
            let pi = std::f32::consts::PI;

            (self.amplitude * f32::sin(2.0 * pi * (1.0 / self.period) * t)) + self.offset
        }
    }

    struct SawTooth {
        period: f32,
        amplitude: f32,
        offset: f32,
    }

    impl SawTooth {
        pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
            Self {
                period,
                amplitude,
                offset,
            }
        }
    }

    impl Wave for SawTooth {
        fn compute(&self, t: f32) -> f32 {
            let now = t % self.period;

            (self.amplitude * (now / self.period)) + self.offset
        }
    }

    struct SquareWave {
        period: f32,
        amplitude: f32,
        offset: f32,
    }

    impl SquareWave {
        pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
            Self {
                period,
                amplitude,
                offset,
            }
        }
    }

    impl Wave for SquareWave {
        fn compute(&self, t: f32) -> f32 {
            let now = t % self.period;
            let state = if now >= (self.period / 2.0) { 1.0 } else { 0.0 };

            (self.amplitude * state) + self.offset
        }
    }
}

#[test]
fn test_graphs() {
    let total = 1_000;
    let x_max = 2.0 * std::f32::consts::PI;

    // let sin = signals::Sine::new(x_max, 1.0, 0.0);
    // let saw_tooth = signals::SawTooth::new(x_max, 1.0, 0.0);
    // let square = signals::SquareWave::new(x_max, 1.0, 0.0);

    assert!(plot_serie(
        (0..total)
            .map(|t| {
                let t = t as f32 * (x_max / (total as f32 - 1.0));
                0.0
                // sin.compute(t)
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
                0.0
                // saw_tooth.compute(t)
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
                0.0
                // square.compute(t)
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
    // let mut system = physical::System::default();

    for t in 0..total {
        let t = t as f32 * (x_max / (total as f32 - 1.0));
        // output.push(system.sensor(t));
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
