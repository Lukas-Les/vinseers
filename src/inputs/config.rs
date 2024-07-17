const DEFAULT_MAX_RESULTS: i32 = -1;
const DEFAULT_RE_PATTERN: &str = "(?i)\\b[A-HJ-NPR-Z0-9]{17}\\b";

pub struct Config {
    target_file_path: Option<String>,
    target_dir: Option<String>,
    output_file: Option<String>,
    max_results: i32,
    re_pattern: String,
}

impl Config {
    pub fn new(
        target_file_path: Option<String>,
        target_dir: Option<String>,
        output_file: Option<String>,
        max_results_opt: Option<u32>,
        re_pattern_opt: Option<String>,
    ) -> Result<Self, String> {
        if target_file_path.is_none() && target_dir.is_none() {
            return Err("Provide only target file or directory, not both!".to_string());
        } 
        if target_file_path.is_some() && target_dir.is_some() {
            return Err("Provide target file or directory!".to_string());
        }
        let max_results: i32 = match max_results_opt {
            Some(v) => v as i32,
            None => DEFAULT_MAX_RESULTS,
        };
        let re_pattern = match re_pattern_opt {
            Some(v) => v,
            None => DEFAULT_RE_PATTERN.to_string(),
        };

        Ok(Self {
            target_file_path,
            target_dir,
            output_file,
            max_results,
            re_pattern,
        })
    }
}
