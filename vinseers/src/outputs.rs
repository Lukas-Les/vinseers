use std::path::Path;

pub fn format(target_file_path: &Path, results: Vec<String>) -> String {
    let path = target_file_path.to_str().unwrap();
    format!("{}->{}", path, results.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let file = Path::new("test.txt");
        let strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = format(&file, strings);
        assert_eq!("test.txt->a b c".to_string(), result);
    }
}
