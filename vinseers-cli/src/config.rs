use vinseers::vid::{VidType, LpnType};

const DEFAULT_MAX_RESULTS: i32 = -1;

#[derive(Debug)]
pub struct Config {
    pub target_file_path: Option<String>,
    pub target_dir: Option<String>,
    pub output_file: Option<String>,
    pub max_results: i32,
    pub vid_type: VidType,
}

impl Config {
    pub fn new(
        target_file_path: Option<String>,
        target_dir: Option<String>,
        output_file: Option<String>,
        max_results_opt: Option<u32>,
        vid_type: VidType,
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

        Ok(Self {
            target_file_path,
            target_dir,
            output_file,
            max_results,
            vid_type,
        })
    }
}

impl TryFrom<Vec<String>> for Config {
    type Error = String;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
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

        Config::new(
            target_file_path,
            target_dir,
            output_file,
            max_results,
            vid_type,
        )
    }
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

    #[test]
    fn test_config_with_target_file_path() {
        let config = Config::new(
            Some("path/to/target/file".to_string()),
            None,
            Some("output/file".to_string()),
            Some(10),
            VidType::Vin,
        )
        .unwrap();

        assert_eq!(
            config.target_file_path,
            Some("path/to/target/file".to_string())
        );
        assert_eq!(config.target_dir, None);
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
    }

    #[test]
    fn test_config_with_target_dir() {
        let config = Config::new(
            None,
            Some("path/to/target/dir".to_string()),
            Some("output/file".to_string()),
            Some(10),
            VidType::Vin,
        )
        .unwrap();

        assert_eq!(config.target_file_path, None);
        assert_eq!(config.target_dir, Some("path/to/target/dir".to_string()));
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, 10);
    }

    #[test]
    fn test_config_default_max_results_and_pattern() {
        let config = Config::new(
            Some("path/to/target/file".to_string()),
            None,
            Some("output/file".to_string()),
            None,
            VidType::Vin,
        )
        .unwrap();

        assert_eq!(
            config.target_file_path,
            Some("path/to/target/file".to_string())
        );
        assert_eq!(config.target_dir, None);
        assert_eq!(config.output_file, Some("output/file".to_string()));
        assert_eq!(config.max_results, DEFAULT_MAX_RESULTS);
    }

    #[test]
    fn test_config_error_both_target_file_and_dir() {
        let config = Config::new(
            Some("path/to/target/file".to_string()),
            Some("path/to/target/dir".to_string()),
            Some("output/file".to_string()),
            Some(10),
            VidType::Vin,
        );

        assert!(config.is_err());
        assert_eq!(
            config.unwrap_err(),
            "Provide only target file or directory, not both!".to_string()
        );
    }

    #[test]
    fn test_config_error_neither_target_file_nor_dir() {
        let config = Config::new(
            None,
            None,
            Some("output/file".to_string()),
            Some(10),
            VidType::Vin,
        );

        assert!(config.is_err());
        assert_eq!(
            config.unwrap_err(),
            "Provide target file or directory!".to_string()
        );
    }
}
