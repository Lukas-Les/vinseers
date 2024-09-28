mod config;

use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use vinseers::parsers::pdf::parse_pdf;
use vinseers::{helpers, outputs, search};

use config::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: Config = Config::try_from(args).expect("Failed to parse arguments");
    if let Err(e) = run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut target_files: Vec<PathBuf> = Vec::new();
    if let Some(v) = &cfg.target_file_path {
        target_files.push(PathBuf::from(v.clone()));
    } else {
        target_files = helpers::walk_directory(Path::new(&cfg.target_dir.unwrap()))
    }
    let mut result: Vec<String> = Vec::new();
    for target_file in target_files {
        let content: String;
        if target_file.extension().unwrap() == "pdf" {
            content = parse_pdf(&target_file).unwrap_or_default();
        } else {
            content = match fs::read_to_string(&target_file) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };
        }
        let matches = search::search(&content, &cfg.vid_type.to_regex());
        if !matches.is_empty() {
            let result_line = outputs::format(&target_file, matches);
            result.push(result_line);
        }
    }
    for line in result.iter() {
        println!("{}", line);
    }
    Ok(())
}
