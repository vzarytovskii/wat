use enum_display::EnumDisplay;

use crate::types::FileView;

pub(super) trait MimeTypeHandler {
    fn handle(file_view: &FileView) -> str;
}

#[derive(Clone, EnumDisplay)]
pub(super) enum PkType {
    Regular,
    Empty,
    Spanned,
    Unkwnown,
}

#[derive(Clone, EnumDisplay)]
pub(super) enum MimeType {
    Unknown,
    Png,
    Elf,
    MZ,
    PK(PkType),
}

pub(super) fn get_magic_mime_type(file_view: &FileView) -> MimeType {
    let view = file_view.view.as_ref();
    match view {
        [0x7f, b'E', b'L', b'F', ..] => MimeType::Elf,
        [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, ..] => MimeType::Png,
        [b'M', b'Z', ..] | [b'Z', b'M', ..] => {
            MimeType::MZ
        },
        [0x50, 0x4B, a, b ] => {
            let pk_type = match (a, b) {
                (0x03, 0x04) => PkType::Regular,
                (0x05, 0x06) => PkType::Empty,
                (0x07, 0x08) => PkType::Spanned,
                _ => PkType::Unkwnown,
            };
            return MimeType::PK(pk_type);
        },
        _ => MimeType::Unknown,
    }
}
