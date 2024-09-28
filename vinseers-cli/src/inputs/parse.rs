use vinseers::vid::{self, LpnType, VidType};

use crate::inputs::config::Config;


pub fn parse_args(args: Vec<String>) -> Result<Config, String> {
    let mut target_file_path: Option<String> = None;
    let mut target_dir: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut max_results: Option<u32> = None;

    let mut vid_type: VidType = VidType::Vin;

    let mut i = 1;
    while i < args.len() - 1 {
        let flag = args[i].as_str();
        if !flag.starts_with("-") {
            i += 1;
            continue;
        }
        let v = Some(args[i + 1].clone());
        match flag {
            "-f" | "--file" => target_file_path = v,
            "-d" | "--dir" => target_dir = v,
            "-o" | "--output" => output_file = v,
            "-m" | "--max" => {
                max_results = Some(v.unwrap().parse::<u32>().map_err(|e| e.to_string())?)
            }
            "--vid" => vid_type = vid_type_from_str(&v.unwrap())?,
            _ => {
                return Err(format!("unknown flag: {}", args[i]));
            }
        }
        i += 2;
    }

    if target_file_path.is_some() && target_dir.is_some() {
        return Err("Provide only target file or directory, not both!".to_string());
    }

    if target_file_path.is_none() && target_dir.is_none() {
        return Err("Provide target file or directory!".to_string());
    }

    Config::new(
        target_file_path,
        target_dir,
        output_file,
        max_results,
        vid_type,
    )
}

fn vid_type_from_str(s: &str) -> Result<VidType, String> {
    match s {
        "vin" => Ok(VidType::Vin),
        "lpn-fin" => Ok(VidType::Lpn(LpnType::Fin)),
        "lpn-fra" => Ok(VidType::Lpn(LpnType::Fra)),
        "lpn-hun" => Ok(VidType::Lpn(LpnType::Hun)),
        "lpn-ita" => Ok(VidType::Lpn(LpnType::Ita)),
        "lpn-ltu" => Ok(VidType::Lpn(LpnType::Ltu)),
        _ => Err(format!("Unknown vid type: {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::config::Config;

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
        ];
        let config: Config = parse_args(args).unwrap();
        assert_eq!(config.target_file_path, Some("path/to/file".to_string()));
        assert_eq!(config.target_dir, None);
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
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
        ];
        let config = parse_args(args).unwrap();
        assert_eq!(config.target_file_path, None);
        assert_eq!(config.target_dir, Some("path/to/dir".to_string()));
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
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
        assert_eq!(
            result.unwrap_err(),
            "Provide only target file or directory, not both!".to_string()
        );
    }

    #[test]
    fn test_parse_args_error_neither_file_nor_dir() {
        let args = vec!["program".to_string()];
        let result = parse_args(args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Provide target file or directory!".to_string()
        );
    }

    #[test]
    fn test_parse_args_unknown_flag() {
        let args = vec![
            "program".to_string(),
            "-f".to_string(),
            "file".to_string(),
            "--unknown".to_string(),
            "un".to_string(),
        ];
        let result = parse_args(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "unknown flag: --unknown".to_string());
    }
}
