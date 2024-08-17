use std::fs;
use std::path::{Path, PathBuf};

pub fn walk_directory(path: &Path) -> Vec<PathBuf> {
    let mut file_names: Vec<PathBuf> = Vec::new();
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        file_names.push(entry_path);
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
        let expected_files = vec![file1_path, file2_path];

        // Check that the result matches the expected output
        assert_eq!(result.len(), 2);
        for file in expected_files {
            assert!(result.contains(&file));
        }
    }
}
