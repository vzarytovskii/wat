mod mime_types;

use crate::analyzers::file_mime_type::mime_types::MIME_TYPES;
use crate::FileView;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;

use self::mime_types::MimeType;

use super::{AnalysisReport, Analyzer};

pub(super) struct FileMimeTypeAnalyzer;

#[async_trait]
impl Analyzer<'_> for FileMimeTypeAnalyzer {
    async fn analyze<'a>(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let extension = file_view
            .path
            .extension()
            .unwrap_or("<NONE>".as_ref())
            .to_str()
            .unwrap_or("<NONE>");
        let mime_type = MIME_TYPES.get(extension).cloned().unwrap_or(MimeType::None);
        let message = format!(
            "{} \n\t{} {} ({} {})",
            "File extension analyzer:".bold().green(),
            "Detected extension:".bold(),
            extension.blue(),
            "type:".bold(),
            mime_type.blue()
        );

        Ok(AnalysisReport { message })
    }
}
