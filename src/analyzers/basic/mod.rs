use crate::FileView;
use color_eyre::Report;

use super::{AnalysisReport, Analyzer};

pub(super) struct BasicAnalyzer;

impl Analyzer<'_> for BasicAnalyzer {
    fn analyze(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let message = format!(
            "Basic file analyzer: \n\tFile length: {} bytes\n\tFile name: \"{}\"",
            file_view.view.len(),
            file_view.path.file_name().unwrap().to_str().unwrap()
        );
        Ok(AnalysisReport { message })
    }
}
