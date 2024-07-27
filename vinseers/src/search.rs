use regex::Regex;

pub fn search(content: &String, re_pattern: &String) -> Vec<String> {
    let re = Regex::new(re_pattern).unwrap();
    re.find_iter(content)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let content = "qwerty WVWZZZ3CZCE024541 pow".to_string();
        let re_pattern = "(?i)\\b[A-HJ-NPR-Z0-9]{17}\\b".to_string();
        assert_eq!(vec!["WVWZZZ3CZCE024541"], search(&content, &re_pattern));
    }
}
