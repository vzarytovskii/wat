mod basic;
mod bom;
mod bytes_distribution;
mod file_extension;

use crate::types::FileView;
use async_trait::async_trait;
use color_eyre::Report;

use futures::StreamExt;

use self::{
    basic::BasicAnalyzer, bom::BomAnalyzer, bytes_distribution::BytesDistributionAnalyzer,
    file_extension::FileExtensionAnalyzer,
};

// A super-simple analysis report that just has a message for the analyzer. Can be some structured data in the future.
// TODO: Should be replace with tui::Widget instead
pub(self) struct AnalysisReport {
    message: String,
}

#[async_trait]
pub(self) trait Analyzer<'a> {
    async fn analyze<'b: 'a>(file_view: &FileView) -> Result<AnalysisReport, Report>;
}

pub(super) async fn analyze(file_view: &FileView<'_>) -> Result<(), Report> {
    // TODO: There must be a better way (codegen?) of instantiating the analyzers, other than manually.
    // ----: Something like going through the file and looking for the analyzer trait and instantiating it.
    // ----: Or maybe a reverse registration, where analyzer registers itself in the registry.
    let analyzers = [
        BasicAnalyzer::analyze,
        BomAnalyzer::analyze,
        BytesDistributionAnalyzer::analyze,
        FileExtensionAnalyzer::analyze,
    ];

    let results = futures::stream::iter(analyzers.into_iter().map(|analyzer| analyzer(file_view)))
        .buffered(3) // TODO: Make configurable?
        .collect::<Vec<_>>()
        .await;

    results.into_iter().for_each(|result| match result {
        Ok(report) => println!("{}", report.message),
        Err(err) => println!("Error: {}", err),
    });

    Ok(())
}
