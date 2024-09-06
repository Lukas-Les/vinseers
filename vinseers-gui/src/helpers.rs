use std::fs;
use std::path::PathBuf;

use vinseers::helpers::walk_directory;
use vinseers::parsers::pdf::parse_pdf;

pub fn process_paths(paths: &Vec<PathBuf>, re_pattern: &str) -> Vec<String> {
    let mut results = Vec::new();
    let all_targets: Vec<PathBuf> = paths
        .iter()
        .flat_map(|pathbuf| {
            if pathbuf.is_dir() {
                walk_directory(pathbuf.as_path())
            } else {
                vec![pathbuf.clone()]
            }
        })
        .collect();

    for path in all_targets.iter() {
        let buffer;
        if path.extension().unwrap() == "pdf" {
            buffer = parse_pdf(path);
        } else {
            if let Ok(file) = fs::read_to_string(path) {
                buffer = Some(file);
            } else {
                buffer = None;
            }
        }
        match buffer {
            Some(v) => {
                let result = vinseers::outputs::format(
                    &path,
                    vinseers::search::search(&v, &re_pattern.to_string()),
                );
                results.push(result);
            }
            None => {
                results.push(path.to_str().unwrap().to_string());
            }
        }
    }
    results
}
