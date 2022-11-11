mod basic;

use crate::types::FileView;
use color_eyre::Report;
use std::fmt::Display;

use self::basic::BasicAnalyzer;

pub(self) trait Analyzer<'a>: Display {
    fn new(file_view: &'a FileView) -> Result<Self, Report>
    where
        Self: Sized;
}

pub(super) fn analyze<'file_view>(file_view: &'file_view FileView) -> Result<(), Report> {
    // TODO: There must be a better way (codegen?) of instantiating the analyzers, other than manually.
    // ----: Something like going through the file and looking for the analyzer trait and instantiating it.
    // ----: Or maybe a reverse regsitration, where analyzer registers itself in the registry.
    let analyzer = BasicAnalyzer::new(file_view)?;

    // TODO: For now, Display trait is used to print the analyzed data, maybe something more structured later.
    print!("{analyzer}");
    Ok(())
}
