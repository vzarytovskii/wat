use crate::FileView;
use async_trait::async_trait;
use bwdraw::DuoPixel;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;
use eyre::{eyre, ContextCompat};
use terminal_size::{terminal_size, Height, Width};

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
            let count = bytecount::count(file_view.view.as_ref(), b) as f32;
            (b, (count / file_len * 100.0) as u8) // Pretty dirty to cast to u8, but it's fine for now.
        });

        let (Width(w), Height(h)) =
            terminal_size().wrap_err_with(|| "Unsupported terminal size!".to_string())?;

        if w < 256 && h < 100 {
            return Err(eyre!("Terminal size is too small!"));
        }

        let width = 256; // includes "borders"
        let height = 100; // includes "borders"

        let mut square = bwdraw::Canvas::new(width, height);

        for (x, y) in distribution {
            let y = 100 - y; // Invert
            let upper = y > 50;
            let lower = y <= 50;
            let y = y / 2;
            //println!("{}: {}", x, y);
            square.mut_set_duopixel(x.into(), y.into(), (upper, lower).into());
            //.wrap_err_with(|| "Failed to set duopixel!")?;
        }

        let message = format!(
            "{}\n{}",
            "Bytes distribution (x = byte(0-256), y = %(0-100)), automatic range for Y axis: "
                .bold()
                .green(),
            square.to_string()
        );

        Ok(AnalysisReport { message })
    }
}
