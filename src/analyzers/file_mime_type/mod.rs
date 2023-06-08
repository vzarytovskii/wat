mod mime_types;

use crate::FileView;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;

use super::{AnalysisReport, Analyzer};

pub(super) struct FileMimeTypeAnalyzer;

#[async_trait]
impl Analyzer<'_> for FileMimeTypeAnalyzer {
    async fn analyze<'a>(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let magic_mime_type = mime_types::get_magic_mime_type(file_view);
        let message = format!(
            "{} \n\t{} {}",
            "File analyzer:".bold().green(),
            "Detected magic type:".bold(),
            magic_mime_type.blue()
        );

        Ok(AnalysisReport { message })
    }
}
