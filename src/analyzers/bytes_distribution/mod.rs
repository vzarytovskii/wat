use crate::analyzer::Analyzer;
use crate::FileView;
use color_eyre::Report;
use std::fmt::Display;

pub(super) struct BytesDistributionAnalyzer<'a> {
    pub(super) file_view: &'a FileView<'a>,
}

impl<'a> BytesDistributionAnalyzer<'a> {
    pub fn new(file_view: &'a FileView) -> Result<Self, Report> {
        Ok(BytesDistributionAnalyzer { file_view })
    }
}

impl<'a> Analyzer for BytesDistributionAnalyzer<'a> {
    fn analyze(&self, file_view: &FileView) -> Result<(), Report> {
        Ok(())
    }
}

impl<'a> Display for BytesDistributionAnalyzer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BytesDistributionAnalyzer")
    }
}
