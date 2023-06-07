use enum_display::EnumDisplay;

use phf::phf_map;

use crate::types::FileView;

#[derive(Clone, EnumDisplay)]
pub enum MimeType {
    Unknown,
    Toml,
    Png,
    Elf,
    MZ
}

#[allow(dead_code)]
pub(super) static MIME_TYPES: phf::Map<&'static str, MimeType> = phf_map! {
    "toml" => MimeType::Toml,
    "png" => MimeType::Png
};

pub(super) fn get_extension_mime_type(extension: &str) -> MimeType {
    MIME_TYPES
        .get(extension)
        .cloned()
        .unwrap_or(MimeType::Unknown)
}

pub(super) fn get_magic_mime_type(file_view: &FileView) -> MimeType {
    let view = file_view.view.as_ref();
    // TODO: Probably want to move to outside of function, otherwise it will be quite big
    match view {
        [0x7f, b'E', b'L', b'F', ..] => MimeType::Elf,
        [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, ..] => MimeType::Png,
        [b'M', b'Z', ..] | [b'Z', b'M', ..] => MimeType::MZ,
        _ => MimeType::Unknown,
    }
}
