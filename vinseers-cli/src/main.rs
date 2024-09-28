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
    let result = helpers::process_paths(&target_files, &cfg.vid_type.to_regex());
    if let Some(v) = &cfg.output_file {
        fs::write(v, result.join("\n"))?;
    } else {
        for r in result.iter() {
            println!("{}", r);
        }
    }
    Ok(())
}
