mod inputs;
mod search;

use std::env;
use std::error::Error;
use std::fs;
use std::process;

use inputs::parse::parse_args;
use inputs::config::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: Config = parse_args(args).expect("Failed configuration!");
    if let Err(e) = run(cfg){
        println!("Application error: {e}");
        process::exit(1);
    }
}


fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let target_files: Vec<String> = Vec::new();
    for target_file in target_files {
        let content = match fs::read_to_string(target_file) {
            Ok(f) => f,
            Err(e) => { 
                eprintln!("{}", e);
                continue; },
        };
    }
    Ok(())
}