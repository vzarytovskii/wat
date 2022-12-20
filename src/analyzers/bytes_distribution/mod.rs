use crate::FileView;
use color_eyre::Report;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;

use super::{AnalysisReport, Analyzer};

pub(super) struct BytesDistributionAnalyzer;

#[async_trait]
impl Analyzer<'_> for BytesDistributionAnalyzer {
    async fn analyze<'a>(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let bytes : [u8; 256] = [0u8; 256];
        let distribution =
            bytes.map(|b| {
                let count = bytecount::count(&file_view.view, b);
                (b, count)
            });
        // TODO: Use some fancier plotting library.
        let message = format!(
            "{}",
            "Bytes distribution analyzer: ".bold().green(),
            );
        Ok(AnalysisReport { message })
    }
}
