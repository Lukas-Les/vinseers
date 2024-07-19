mod inputs;
mod search;
mod outputs;

use std::env;
use std::error::Error;
use std::fs;
use std::process;

use inputs::parse::parse_args;
use inputs::config::Config;
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
