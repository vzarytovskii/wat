use std::cmp;

use crate::FileView;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;
use textplots::{Chart, Plot, Shape};
use terminal_size::{Width, Height, terminal_size};

use super::{AnalysisReport, Analyzer};

pub(super) struct BytesDistributionAnalyzer;

#[async_trait]
// TODO: Positional bytes distribution analyzer (which bytes are located at which positions in the file).
impl Analyzer<'_> for BytesDistributionAnalyzer {
    async fn analyze<'a>(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let bytes: [u8; 256] = (0..=u8::MAX)
            .collect::<Vec<_>>()
            .try_into()
            .expect("wrong size iterator");
        let file_len = file_view.view.len() as f32;
        let distribution = bytes.map(|b| {
            let count = bytecount::count(&file_view.view.as_ref(), b) as f32;
            (b as f32, count / file_len * 100.0)
        });

        let (Width(w), Height(h)) = terminal_size().expect("Unable to get terminal size");
        let width = cmp::max(w, 32);
        let height = cmp::max(h, 32);
        // TODO: For now, it's a simple chart, but it should be a widget that can be rendered in the tui.
        let chart =
            Chart::new(width.into(), height.into(), 0.0, 255.0)
                .lineplot(&Shape::Bars(&distribution))
                .to_string();
        let message = format!(
            "{}\n{  }",
            "Bytes distribution (x = byte, y = %), automatic range for Y axis: ".bold().green(),
            chart);

        Ok(AnalysisReport { message })
    }
}
