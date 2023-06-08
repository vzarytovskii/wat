pub(super) struct MzMimeTypeHandler;

impl MimeTypeHandler for MzMimeTypeHandler {
    fn handle(file_view: &FileView) -> str {
        return "MZ";
    }
}
