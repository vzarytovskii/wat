use enum_display::EnumDisplay;

use crate::types::FileView;

pub(super) trait MimeTypeHandler {
    fn handle(file_view: &FileView) -> MimeType;
}

#[derive(Clone, EnumDisplay)]
pub(super) enum MimeType {
    Unknown,
    Png,
    Elf,
    MZ,
    PK,
}

pub(super) fn get_magic_mime_type(file_view: &FileView) -> MimeType {
    let view = file_view.view.as_ref();
    match view {
        [0x7f, b'E', b'L', b'F', ..] => MimeType::Elf,
        [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, ..] => MimeType::Png,
        [b'M', b'Z', ..] => MimeType::MZ,
        [0x50, 0x4B, _a, _b ] => MimeType::PK,
        _ => MimeType::Unknown,
    }
}
