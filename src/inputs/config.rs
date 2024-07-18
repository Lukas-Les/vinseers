const DEFAULT_MAX_RESULTS: i32 = -1;
const DEFAULT_RE_PATTERN: &str = "(?i)\\b[A-HJ-NPR-Z0-9]{17}\\b";


#[derive(Debug)]
pub struct Config {
    pub target_file_path: Option<String>,
    pub target_dir: Option<String>,
    pub output_file: Option<String>,
    pub max_results: i32,
    pub re_pattern: String,
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
            return Err("Provide target file or directory!".to_string());
        } 
        if target_file_path.is_some() && target_dir.is_some() {
            return Err("Provide only target file or directory, not both!".to_string());
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_with_target_file_path() {
        let config = Config::new(
            Some("path/to/target/file".to_string()), 
            None, 
            Some("output/file".to_string()), 
            Some(10), 
            Some("\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string())
        ).unwrap();

        assert_eq!(config.target_file_path, Some("path/to/target/file".to_string()));
        assert_eq!(config.target_dir, None);
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
        assert_eq!(config.re_pattern, "\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string());
    }

    #[test]
    fn test_config_with_target_dir() {
        let config = Config::new(
            None, 
            Some("path/to/target/dir".to_string()), 
            Some("output/file".to_string()), 
            Some(10), 
            Some("\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string())
        ).unwrap();

        assert_eq!(config.target_file_path, None);
        assert_eq!(config.target_dir, Some("path/to/target/dir".to_string()));
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
        assert_eq!(config.re_pattern, "\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string());
    }

    #[test]
    fn test_config_default_max_results_and_pattern() {
        let config = Config::new(
            Some("path/to/target/file".to_string()), 
            None, 
            Some("output/file".to_string()), 
            None, 
            None
        ).unwrap();

        assert_eq!(config.target_file_path, Some("path/to/target/file".to_string()));
        assert_eq!(config.target_dir, None);
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, DEFAULT_MAX_RESULTS);
        assert_eq!(config.re_pattern, DEFAULT_RE_PATTERN.to_string());
    }

    #[test]
    fn test_config_error_both_target_file_and_dir() {
        let config = Config::new(
            Some("path/to/target/file".to_string()), 
            Some("path/to/target/dir".to_string()), 
            Some("output/file".to_string()), 
            Some(10), 
            Some("\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string())
        );

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "Provide only target file or directory, not both!".to_string());
    }

    #[test]
    fn test_config_error_neither_target_file_nor_dir() {
        let config = Config::new(
            None, 
            None, 
            Some("output/file".to_string()), 
            Some(10), 
            Some("\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string())
        );

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "Provide target file or directory!".to_string());
    }
}
