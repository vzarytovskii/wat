mod basic;
mod bom;

use crate::types::FileView;
use color_eyre::Report;

use self::{basic::BasicAnalyzer, bom::BomAnalyzer};

// A super-simple analysis report that just has a message for the analyzer. Can be some structured data in the future.
pub(self) struct AnalysisReport {
    message: String,
}

pub(self) trait Analyzer<'a> {
    fn analyze(file_view: &'a FileView) -> Result<AnalysisReport, Report>;
}

pub(super) async fn analyze(file_view: &FileView<'_>) -> Result<(), Report> {
    // TODO: There must be a better way (codegen?) of instantiating the analyzers, other than manually.
    // ----: Something like going through the file and looking for the analyzer trait and instantiating it.
    // ----: Or maybe a reverse registration, where analyzer registers itself in the registry.
    let analyzers = [BasicAnalyzer::analyze, BomAnalyzer::analyze];
    for analyzer in analyzers.iter() {
        let report = analyzer(file_view)?;
        println!("{}", report.message);
    }
    Ok(())
}
