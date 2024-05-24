use crate::FileView;
use async_trait::async_trait;
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

        // TODO: This needs to be calculated differently, since it will yield <1% for big files.
        // TODO: Calculate file entropy as well
        let distribution = bytes.map(|b| {
            let count = bytecount::count(file_view.view.as_ref(), b) as f32;
            println!(
                "{}: {} (calculated: {}, file_len: {})",
                b,
                count,
                (count / file_len * 100.) as u8,
                file_len
            );
            (b, (count / file_len * 100.) as u8) // Pretty dirty to cast to u8, but it's fine for now.
        });

        let (Width(w), Height(h)) =
            terminal_size().wrap_err_with(|| "Unsupported terminal size!".to_string())?;

        if w < 256 && h < 100 {
            return Err(eyre!("Terminal size is too small!"));
        }

        let width = 256;
        let height = 101;

        let mut canvas = bwdraw::Canvas::new(width, height);

        for (x, y) in distribution {
            let y = 100 - y; // Invert
            canvas.mut_set(x.into(), y.into(), true);
        }
        let message = format!(
            "{}\n{}\n{}\n{}{}{}",
            "Bytes distribution (x = byte(0-256), y = %(0-100)): "
                .bold()
                .green(),
            "100%",
            canvas.to_string().blue(),
            0.green().bold(),
            " ".repeat(255),
            255.green().bold(),

        );

        Ok(AnalysisReport { message })
    }
}
