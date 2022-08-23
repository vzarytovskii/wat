use color_eyre::Report;

use crate::FileView;

pub(super) struct BytesDistributionAnalyzer<'a> {
    pub(super) file_info: &'a FileView<'a>,
}

impl<'a> BytesDistributionAnalyzer<'a> {
    pub fn new(file_info: &'a FileView) -> Result<Self, Report> {
        Ok(BytesDistributionAnalyzer { file_info })
    }
}
