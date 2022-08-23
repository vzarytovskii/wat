use crate::types::FileView;
use color_eyre::Report;
use std::fmt::Display;

pub(crate) trait Analyzer: Display {
    fn analyze(&self, file_view: &FileView) -> Result<(), Report>;
}
