use enum_display::EnumDisplay;

use phf::phf_map;

#[derive(Clone, EnumDisplay)]
pub enum MimeType {
    None,
    Toml,
    Png,
}

#[allow(dead_code)]
pub(super) static MIME_TYPES: phf::Map<&'static str, MimeType> = phf_map! {
    "toml" => MimeType::Toml,
    "png" => MimeType::Png
};
