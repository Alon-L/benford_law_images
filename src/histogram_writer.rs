pub mod histogram {
    use plotters::prelude::*;

    pub struct HistogramWriter<'a> {
        name: &'a str,
        data: &'a Vec<u32>,
    }

    impl<'a> HistogramWriter<'a> {
        pub fn new(name: &'a str, data: &'a Vec<u32>) -> HistogramWriter<'a> {
            HistogramWriter { name, data }
        }

        pub fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
            let root = BitMapBackend::new(path, (1200, 800)).into_drawing_area();

            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(50)
                .y_label_area_size(80)
                .margin(5)
                .caption(self.name, ("DejaVu Sans", 50).into_font())
                .build_ranged(1u32..10u32, 0u32..100u32)?;

            chart
                .configure_mesh()
                .disable_x_mesh()
                .line_style_1(&WHITE.mix(0.5))
                .x_label_offset(60)
                .y_desc("Percentage")
                .x_desc("Digit")
                .axis_desc_style(("DejaVu Sans", 15).into_font())
                .draw()?;

            chart.draw_series(
                Histogram::vertical(&chart)
                    .style(RED.mix(0.5).filled())
                    .data(self.data.iter().map(|x: &u32| (*x, 1))),
            )?;

            Ok(())
        }
    }
}
