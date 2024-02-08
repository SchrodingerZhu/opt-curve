use plotters::prelude::*;
use rand::prelude::*;

#[path = "lib.rs"]
mod lib;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let trace: Vec<usize> = (0..1024).map(|_| rng.gen_range(0..256)).collect();

    let root = BitMapBackend::new("plots-img/opt-miss-ratio-curve-plot.png", (640, 480))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("OPT Miss Ratio Curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..256f64, 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (1..257)
                .map(|x| x)
                .map(|x| (x as f64, lib::opt_miss_ratio(&trace, x))),
            &RED,
        ))?
        .label("OPT Miss Ratio Curve")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
