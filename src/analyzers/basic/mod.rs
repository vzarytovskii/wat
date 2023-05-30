use crate::FileView;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;
use human_repr::HumanCount;
use std::fs;

use super::{AnalysisReport, Analyzer};

pub(super) struct BasicAnalyzer;

#[async_trait]
impl Analyzer<'_> for BasicAnalyzer {
    async fn analyze<'a>(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let message = format!(
            "{}\n\t{} {} ({})\n\t{} {}\n\t{} {}\n\t{} {}",
            "Basic file analyzer:".bold().green(),
            "File length:".bold(),
            file_view.view.len().human_count_bytes().blue(),
            file_view.view.len().blue(),
            "File name:".bold(),
            file_view.path.file_name().unwrap().to_str().unwrap().blue(),
            "Full path:".bold(),
            fs::canonicalize(file_view.path)
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .blue(),
            "Is read-only:".bold(),
            file_view.path.metadata()?.permissions().readonly().blue()
        );
        Ok(AnalysisReport { message })
    }
}
