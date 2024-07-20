mod inputs;
mod search;
mod outputs;

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

use inputs::config::Config;
use inputs::parse::parse_args;
use search::search;
use outputs::outputs::format;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: Config = parse_args(args).expect("Failed configuration");
    if let Err(e) = run(cfg){
        println!("Application error: {e}");
        process::exit(1);
    }
}


fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut target_files: Vec<String> = Vec::new();
    if let Some(v) = &cfg.target_file_path {
        target_files.push(v.clone());
    } else {
        target_files = walk_directory(Path::new(&cfg.target_dir.unwrap()))
    }

    let mut result: Vec<String> = Vec::new();
    for target_file in target_files {
        let content = match fs::read_to_string(&target_file) {
            Ok(f) => f,
            Err(e) => { 
                eprintln!("{}", e);
                continue; },
        };
        let matches = search(&content, &cfg.re_pattern);
        if !matches.is_empty() {
            let result_line = format(&target_file, matches);
            result.push(result_line);
        }
    }
    for line in result.iter() {
        println!("{}", line);
    }
    Ok(())
}


fn walk_directory(path: &Path) -> Vec<String> {
    let mut file_names = Vec::new();
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(file_name) = entry_path.to_str() {
                            file_names.push(file_name.to_string());
                        }
                    } else if entry_path.is_dir() {
                        let mut child_files = walk_directory(&entry_path);
                        file_names.append(&mut child_files);
                    }
                }
            }
        }
    }
    file_names
}
