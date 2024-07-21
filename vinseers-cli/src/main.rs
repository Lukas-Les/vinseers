mod inputs;

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

use vinseers::{outputs, search};

use inputs::config::Config;
use inputs::parse::parse_args;



fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: Config = parse_args(args).expect("Failed configuration");
    if let Err(e) = run(cfg){
        eprintln!("Application error: {e}");
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
        let matches = search::search(&content, &cfg.re_pattern);
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


#[cfg(test)]
mod tests {
    use super::walk_directory;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_walk_directory() {
        // Create a temporary directory
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();

        // Create some files and directories inside the temporary directory
        let file1_path = temp_path.join("file1.txt");
        File::create(&file1_path).expect("Failed to create file1");

        let sub_dir = temp_path.join("sub_dir");
        fs::create_dir(&sub_dir).expect("Failed to create sub_dir");

        let file2_path = sub_dir.join("file2.txt");
        File::create(&file2_path).expect("Failed to create file2");

        // Run the walk_directory function
        let result = walk_directory(temp_path);

        // Convert the paths to strings for comparison
        let expected_files = vec![
            file1_path.to_str().unwrap().to_string(),
            file2_path.to_str().unwrap().to_string(),
        ];

        // Check that the result matches the expected output
        assert_eq!(result.len(), 2);
        for file in expected_files {
            assert!(result.contains(&file));
        }
    }
}
