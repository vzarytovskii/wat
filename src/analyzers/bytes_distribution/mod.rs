use crate::FileView;
use color_eyre::Report;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;

use super::{AnalysisReport, Analyzer};

pub(super) struct BytesDistributionAnalyzer;

#[async_trait]
// TODO: Positional bytes distribution analyzer (which bytes are located at which positions in the file).
impl Analyzer<'_> for BytesDistributionAnalyzer {
    async fn analyze<'a>(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let bytes : [u8; 256] = (0..=u8::MAX).collect::<Vec<_>>().try_into().expect("wrong size iterator");
        let file_len = file_view.view.len() as f64;
        let distribution =
            bytes.map(|b| {
                let count  = bytecount::count(&file_view.view.as_ref(), b)  as f64;
                (format!("{:#04X?}", b), count / file_len * 100.0)
            });
        let message = format!(
            "{}\n{:?}",
            "Bytes distribution analyzer: ".bold().green(),
            distribution);
        Ok(AnalysisReport { message })
    }
}
