pub fn format(target_file_path: &str, results: Vec<String>) -> String {
    format!("{}->{}", target_file_path, results.join(" "))
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::format;

    #[test]
    fn test_format() {
        let file = "test.txt";
        let strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = format(file, strings);
        assert_eq!("test.txt->a b c".to_string(), result);
    }
}
