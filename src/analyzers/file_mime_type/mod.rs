mod mime_types;

use crate::FileView;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;

use self::mime_types::get_extension_mime_type;

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
        let extension_mime_type = get_extension_mime_type(extension);
        let magic_mime_type = mime_types::get_magic_mime_type(file_view);
        let message = format!(
            "{} \n\t{} {} ({} {})\n\t{} {}",
            "File extension analyzer:".bold().green(),
            "Detected extension:".bold(),
            extension.blue(),
            "type:".bold(),
            extension_mime_type.blue(),
            "Detected magic type:".bold(),
            magic_mime_type.blue()
        );

        Ok(AnalysisReport { message })
    }
}
