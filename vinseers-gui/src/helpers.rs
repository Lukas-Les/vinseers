use std::fs::{read_dir, File};
use std::io::{self, Read};
use std::path::PathBuf;

use iced::widget::text_editor;

pub fn process_paths_recursive(paths: &Option<Vec<PathBuf>>, re_pattern: &str) -> Vec<String> {
    let mut results = Vec::new();

    if let Some(paths) = paths {
        for path in paths {
            let path = path.as_path();
            if path.is_dir() {
                println!("Processing directory: {}", path.display());
                if let Ok(entries) = read_dir(path) {
                    let mut sub_paths = Vec::new();
                    for entry in entries {
                        if let Ok(entry) = entry {
                            sub_paths.push(entry.path());
                        }
                    }
                    results.extend(process_paths_recursive(&Some(sub_paths), re_pattern));
                }
            } else {
                println!("Processing file: {}", path.display());
                if let Ok(mut file) = File::open(path) {
                    let mut buffer = String::new();
                    if file.read_to_string(&mut buffer).is_ok() {
                        let path_string = path.to_str().unwrap_or_default().to_string();
                        let result = vinseers::outputs::format(
                            &path_string,
                            vinseers::search::search(&buffer, &re_pattern.to_string()),
                        );
                        results.push(result);
                    } else {
                        println!("Failed to read file: {}", path.display());
                    }
                } else {
                    println!("Failed to open file: {}", path.display());
                }
            }
        }
    }
    results
}
