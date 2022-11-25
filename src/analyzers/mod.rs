mod basic;

use crate::types::FileView;
use color_eyre::Report;

use self::basic::BasicAnalyzer;

// A super-simple analysis report that just has a message for the analyzer. Can be some structured data in the future.
pub(self) struct AnalysisReport {
    message: String,
}

pub(self) trait Analyzer<'a> {
    fn analyze(file_view: &'a FileView) -> Result<AnalysisReport, Report>;
}

pub(super) fn analyze<'file_view>(file_view: &'file_view FileView) -> Result<(), Report> {
    // TODO: There must be a better way (codegen?) of instantiating the analyzers, other than manually.
    // ----: Something like going through the file and looking for the analyzer trait and instantiating it.
    // ----: Or maybe a reverse regsitration, where analyzer registers itself in the registry.
    let analysis_report = BasicAnalyzer::analyze(file_view)?;

    print!("{0}", analysis_report.message);
    Ok(())
}
