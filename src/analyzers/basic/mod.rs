use crate::FileView;
use color_eyre::Report;
use std::fmt::Display;

use super::Analyzer;

pub(super) struct BasicAnalyzer {
    file_name: String,
    file_length: usize,
}

impl<'a> Analyzer<'a> for BasicAnalyzer {
    fn new(file_view: &'a FileView) -> Result<Self, Report> {
        let file_length = file_view.view.len();
        let file_name = file_view
            .path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Ok(BasicAnalyzer {
            file_length: file_length,
            file_name: file_name,
        })
    }
}

impl<'a> Display for BasicAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Basic file analyzer: \n\tFile length: {} bytes\n\tFile name: \"{}\"",
            self.file_length, self.file_name
        )
    }
}
