use regex::Regex;
use std::process::Command;

pub fn search((search_key, search_value): (String, String)) -> Result<String, String> {
    let output = Command::new("pactl").arg("list").output().unwrap();

    if !output.status.success() {
        println!("error");
    }

    let pattern = Regex::new(&format!(".*{}.*", search_value).to_owned()).unwrap();

    let text = String::from_utf8(output.stdout).unwrap();

    for group in text.split_terminator("\n\n") {
        let mut lines = group.lines();
        let mut first_line = lines.next().unwrap().split(" #");
        let group_type = first_line.next().unwrap();
        let id = String::from(first_line.next().unwrap());

        if group_type != "Sink" {
            continue;
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
    }

    Err(String::from("Not found"))
}
