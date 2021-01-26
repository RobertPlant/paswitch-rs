use regex::{Regex, RegexBuilder};
use std::process::Command;

fn list_sinks() -> String {
    let output = Command::new("pactl").arg("list").output().unwrap();

    if !output.status.success() {
        println!("error");
    }

    String::from_utf8(output.stdout).unwrap()
}

fn get_search_pattern(search_value: String, case_sensitive: bool) -> Result<Regex, regex::Error> {
    RegexBuilder::new(&format!(".*{}.*", search_value).to_owned())
        .case_insensitive(!case_sensitive)
        .build()
}

pub fn search(
    search_key: String,
    search_value: String,
    case_sensitive: bool,
) -> Result<String, String> {
    let pattern = get_search_pattern(search_value, case_sensitive)
        .expect("Something went wrong building the regex");

    for group in list_sinks().split_terminator("\n\n") {
        match find(group, search_key.to_owned(), pattern.to_owned()) {
            Ok(id) => return Ok(id),
            _ => continue,
        }
    }

    Err(String::from("Not found"))
}

fn find(group: &str, search_key: String, pattern: Regex) -> Result<String, String> {
    let mut lines = group.lines();
    let mut first_line = lines.next().unwrap().split(" #");
    let group_type = first_line.next().unwrap();
    let id = String::from(first_line.next().unwrap());

    if group_type != "Sink" {
        return Err("Not a Sink".to_string());
    }

    for line in lines {
        let mut split_line = line.split(": ");
        let key = split_line.next().unwrap().trim();
        let value = split_line.next().unwrap_or("");

        if key == search_key && pattern.is_match(value) {
            return Ok(id);
        }
    }

    Err("Not matched".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_by_description() {
        let search_value = "Fiio".to_string();
        let pattern = Regex::new(&format!(".*{}.*", search_value).to_owned()).unwrap();
        let contents = fs::read_to_string("src/test/data/pactl-fiio.txt")
            .expect("Something went wrong reading the file");

        assert_eq!(
            find(&contents, "Description".to_string(), pattern).unwrap(),
            "43"
        )
    }

    #[test]
    fn test_get_search_pattern_case_sensitive() {
        assert!(get_search_pattern("test".to_string(), true)
            .unwrap()
            .is_match("test"))
    }

    #[test]
    fn test_get_search_pattern_case_sensitive_with_capitals() {
        assert!(!get_search_pattern("Test".to_string(), true)
            .unwrap()
            .is_match("test"))
    }

    #[test]
    fn test_get_search_pattern_case_insensitive() {
        assert!(get_search_pattern("test".to_string(), false)
            .unwrap()
            .is_match("Test"))
    }

    #[test]
    fn test_get_search_pattern_case_insensitive_with_capitals() {
        assert!(get_search_pattern("Test".to_string(), false)
            .unwrap()
            .is_match("test"))
    }
}
