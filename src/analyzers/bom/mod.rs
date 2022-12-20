use crate::FileView;
use color_eyre::Report;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;

use super::{AnalysisReport, Analyzer};

/// https://en.wikipedia.org/wiki/Byte_order_mark#Byte_order_marks_by_encoding
enum Mark { None, Bocu1, Gb18030, Scsu, Utf1, Utf16Be, Utf16Le, Utf32Be, Utf32Le, Utf7, Utf8, UtfEbcdic, }
impl AsRef<str> for Mark {
    fn as_ref(&self) -> &str {
        match self {
            Mark::None => "None",
            Mark::Bocu1 => "BOCU-1",
            Mark::Gb18030 => "GB-18030",
            Mark::Scsu => "SCSU",
            Mark::Utf1 => "UTF-1",
            Mark::Utf16Be => "UTF-16 Big Endian",
            Mark::Utf16Le => "UTF-16 Little Endian",
            Mark::Utf32Be => "UTF-32 Big Endian",
            Mark::Utf32Le => "UTF-32 Little Endian",
            Mark::Utf7 => "UTF-7",
            Mark::Utf8 => "UTF-8 (BOM)",
            Mark::UtfEbcdic => "UTF-EBCDIC",
        }
    }
}
impl From<&FileView<'_>> for Mark {
    fn from(file_view: &FileView) -> Self {
        let view = file_view.view.as_ref();
        match view {
            &[0xfb, 0xee, 0x28, .. ] => Mark::Bocu1,
            &[0x84, 0x31, 0x95, 0x33, .. ] => Mark::Gb18030,
            &[0x0e, 0xfe, 0xff, .. ] => Mark::Scsu,
            &[0xf7, 0x64, 0x4c, .. ] => Mark::Utf1,
            &[0xfe, 0xff, .. ] => Mark::Utf16Be,
            &[0xff, 0xfe, 0x0, 0x0, .. ] => Mark::Utf32Le,
            &[0xff, 0xfe, .. ] =>Mark::Utf16Le,
            &[0x0, 0x0, 0xfe, 0xff, .. ] => Mark::Utf32Be,
            &[0x2b, 0x2f, 0x76, 0x38 | 0x39 | 0x2b | 0x2f, ..]  => Mark::Utf7,
            &[0xef, 0xbb, 0xbf, .. ] => Mark::Utf8,
            &[0xdd, 0x73, 0x66, 0x73, .. ] => Mark::UtfEbcdic,
            _ => Mark::None
        }
    }
}

pub(super) struct BomAnalyzer;

#[async_trait]
impl Analyzer<'_> for BomAnalyzer {
    async fn analyze<'a>(file_view: &FileView) -> Result<AnalysisReport, Report> {
        let mark: Mark = file_view.into();
        let message = format!(
            "{} \n\t{} {}",
            "BOM analyzer:".bold().green(),
            "Detected BOM:".bold(),
            mark.as_ref().blue());

        Ok(AnalysisReport { message })
    }
}
