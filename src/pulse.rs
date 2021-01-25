use regex::Regex;
use std::process::Command;

fn list_sinks() -> String {
    let output = Command::new("pactl").arg("list").output().unwrap();

    if !output.status.success() {
        println!("error");
    }

    String::from_utf8(output.stdout).unwrap()
}

pub fn search(search_key: String, search_value: String) -> Result<String, String> {
    let pattern = Regex::new(&format!(".*{}.*", search_value).to_owned()).unwrap();

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

        if key == search_key {
            let found = pattern.find(value);

            if found.is_some() {
                return Ok(id);
            }
        }
    }

    Err("Not matched".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn find_by_description() {
        let search_value = "Fiio".to_string();
        let pattern = Regex::new(&format!(".*{}.*", search_value).to_owned()).unwrap();
        let contents = fs::read_to_string("src/test/data/pactl-fiio.txt")
            .expect("Something went wrong reading the file");

        assert_eq!(
            find(&contents, "Description".to_string(), pattern).unwrap(),
            "43"
        )
    }
}
