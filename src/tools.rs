pub fn f(x: f32) -> f32 {
    let position = 128.0;
    if x != position {
        (x - position).sin() * 100.0 / (x - position).abs()
    } else {
        100.0
    }
}

pub fn plot(data: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;

    let max = *data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let root = BitMapBackend::new("test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(data.len() as f32- 100f32..data.len() as f32, -1f32..max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(x, y)| (x as f32, *y)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
