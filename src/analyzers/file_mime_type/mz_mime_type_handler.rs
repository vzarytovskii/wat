use super::mime_types::{MimeType, MimeTypeHandler};
use crate::types::FileView;

pub(super) struct MzMimeTypeHandler;

impl MimeTypeHandler for MzMimeTypeHandler {
    fn handle(_file_view: &FileView) -> MimeType {
        MimeType::MZ
    }
}
