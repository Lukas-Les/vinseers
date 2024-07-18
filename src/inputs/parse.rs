use crate::inputs::config::Config;


pub fn parse_args(args: Vec<String>) -> Result<Config, String> {
    let mut target_file_path: Option<String> = None;
    let mut target_dir: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut max_results: Option<u32> = None;
    let mut re_pattern: Option<String> = None;

    let mut i = 1;
    while i < args.len() - 1 {
        let v = Some(&args[i+1]).cloned();
        match args[i].as_str() {
            "-f" | "--file" => { target_file_path = v },
            "-d" | "--dir" => { target_dir = v },
            "-o" | "--output" => { output_file = v},
            "-m" | "--max" => { max_results = Some(v.unwrap().parse::<u32>().unwrap()) },
            "-r" | "--re" => { re_pattern = v },
            _ => {
                panic!("unknown flag: {}", args[i])
            }
        }
        i += 1;
    }
    Config::new(target_file_path, target_dir, output_file, max_results, re_pattern)
    }

    #[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::config::{Config};

    #[test]
    fn test_parse_args_with_file() {
        let args = vec![
            "program".to_string(),
            "-f".to_string(),
            "path/to/file".to_string(),
            "-o".to_string(),
            "output/file".to_string(),
            "-m".to_string(),
            "10".to_string(),
            "-r".to_string(),
            "\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string(),
        ];
        let config: Config = parse_args(args).unwrap();
        assert_eq!(config.target_file_path, Some("path/to/file".to_string()));
        assert_eq!(config.target_dir, None);
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
        assert_eq!(config.re_pattern, "\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string());
    }

    #[test]
    fn test_parse_args_with_dir() {
        let args = vec![
            "program".to_string(),
            "-d".to_string(),
            "path/to/dir".to_string(),
            "-o".to_string(),
            "output/file".to_string(),
            "-m".to_string(),
            "10".to_string(),
            "-r".to_string(),
            "\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string(),
        ];
        let config = parse_args(args).unwrap();
        assert_eq!(config.target_file_path, None);
        assert_eq!(config.target_dir, Some("path/to/dir".to_string()));
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
        assert_eq!(config.re_pattern, "\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string());
    }

    #[test]
    fn test_parse_args_with_defaults() {
        let args = vec![
            "program".to_string(),
            "-f".to_string(),
            "path/to/file".to_string(),
            "-o".to_string(),
            "output/file".to_string(),
        ];
        let config = parse_args(args).unwrap();
        assert_eq!(config.target_file_path, Some("path/to/file".to_string()));
        assert_eq!(config.target_dir, None);
        assert_eq!(config.output_file, Some("output/file".to_string()));
    }

    #[test]
    fn test_parse_args_error_both_file_and_dir() {
        let args = vec![
            "program".to_string(),
            "-f".to_string(),
            "path/to/file".to_string(),
            "-d".to_string(),
            "path/to/dir".to_string(),
        ];
        let result = parse_args(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Provide only target file or directory, not both!".to_string());
    }

    #[test]
    fn test_parse_args_error_neither_file_nor_dir() {
        let args = vec![
            "program".to_string(),
        ];
        let result = parse_args(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Provide target file or directory!".to_string());
    }

    #[test]
    fn test_parse_args_unknown_flag() {
        let args = vec![
            "program".to_string(),
            "--unknown".to_string(),
        ];
        let result = std::panic::catch_unwind(|| parse_args(args));
        assert!(result.is_err());
    }
}
