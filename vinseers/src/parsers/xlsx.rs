use std::path::Path;

use calamine::{open_workbook, DataType, Reader, Xlsx};


/// Tries to read all text from all sheets;
pub fn parse_xlsx(file_path: &Path) -> Option<String> {
    let mut workbook: Xlsx<_> = match open_workbook(file_path) {
        Ok(w) => w,
        _  => {return None;}
    };

    let mut result = String::new();
    
    for sheet_name in workbook.sheet_names().to_owned() {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            for row in range.rows() {
                for cell in row {
                    if let Some(v) = cell.as_string() {
                        result.push_str(v.as_str());
                        result.push('\t');
                    }
                }
                result.push('\n');
            }
        }
    }
    Some(result)
}
