use std::path::PathBuf;

use pdf_extract;


/// This function takes pdf file path and returs parsed text.
pub fn parse_pdf(file_path: &PathBuf) -> Option<String> {
    match pdf_extract::extract_text(file_path) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
