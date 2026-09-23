use crate::commands::Type;
use anyhow::{anyhow, Result};
use std::io::{stdout, Write};
use std::process::Command;
use std::str::FromStr;
use std::str::Lines;

#[derive(Debug, PartialEq)]
pub enum EntityType {
    Sink,
    Module,
    Source,
    Input,
    SinkInput,
    Client,
    Card,
    Unknown,
}

impl FromStr for EntityType {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<EntityType, Self::Err> {
        match input {
            "Sink" => Ok(EntityType::Sink),
            "Module" => Ok(EntityType::Module),
            "Source" => Ok(EntityType::Source),
            "Input" => Ok(EntityType::Input),
            "Sink Input" => Ok(EntityType::SinkInput),
            "Client" => Ok(EntityType::Client),
            "Card" => Ok(EntityType::Card),
            _ => Ok(EntityType::Unknown),
        }
    }
}

struct Entity {
    id: String,
    state: String,
    name: String,
    description: String,
    driver: String,
    mute: String,
    volume: String,
}

fn list_sinks() -> String {
    let output = Command::new(Type::Pactl.to_string())
        .arg("list")
        .output()
        .unwrap();

    if !output.status.success() {
        println!("error");
    }

    String::from_utf8(output.stdout).unwrap()
}

/// Literal substring match. Device descriptions routinely contain regex
/// metacharacters -- "Arctis 7 (Game)" parsed as a pattern matches the text
/// "Arctis 7 Game" and misses the real name -- so the needle is compared as
/// plain text.
fn matches(haystack: &str, needle: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        haystack.contains(needle)
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

pub fn search(search_key: String, search_value: String, case_sensitive: bool) -> Result<String> {
    for group in list_sinks().split_terminator("\n\n") {
        match find(group, search_key.to_owned(), &search_value, case_sensitive) {
            Ok(id) => return Ok(id),
            _ => continue,
        }
    }

    Err(anyhow!("Search failed: no matching sink found"))
}

fn find(group: &str, search_key: String, needle: &str, case_sensitive: bool) -> Result<String> {
    let mut lines = group.lines();
    let mut first_line = lines.next().unwrap().split(" #");
    let group_type = EntityType::from_str(first_line.next().unwrap())?;
    let id = String::from(first_line.next().unwrap());

    if group_type != EntityType::Sink {
        return Err(anyhow!("Not a Sink"));
    }

    for line in lines {
        let mut split_line = line.split(": ");
        let key = split_line.next().unwrap().trim();
        let value = split_line.next().unwrap_or("");

        if key == search_key && matches(value, needle, case_sensitive) {
            return Ok(id);
        }
    }

    Err(anyhow!("Not matched"))
}

pub fn list() -> Result<()> {
    let mut t = stdout();
    writeln!(t).unwrap();

    for group in list_sinks().split_terminator("\n\n") {
        let mut lines = group.lines();
        let mut first_line = lines.next().unwrap().split(" #");
        let group_type = EntityType::from_str(first_line.next().unwrap())?;
        let id = String::from(first_line.next().unwrap());

        if group_type != EntityType::Sink {
            continue;
        }

        let sink = Entity {
            id,
            state: pull_data(&mut lines, "State".to_string())?,
            name: pull_data(&mut lines, "Name".to_string())?,
            description: pull_data(&mut lines, "Description".to_string())?,
            driver: pull_data(&mut lines, "Driver".to_string())?,
            mute: pull_data(&mut lines, "Mute".to_string())?,
            volume: pull_data(&mut lines, "Volume".to_string())?,
        };

        print_attribute(&mut t, "         ID", &sink.id);
        print_attribute(&mut t, "Description", &sink.description);
        print_attribute(&mut t, "       Name", &sink.name);
        print_attribute(&mut t, "      State", &sink.state);
        print_attribute(&mut t, "     Driver", &sink.driver);
        print_attribute(&mut t, "       Mute", &sink.mute);
        print_attribute(&mut t, "     Volume", &sink.volume);
        writeln!(t).unwrap();
    }

    Ok(())
}

fn pull_data(lines: &mut Lines, search_key: String) -> Result<String> {
    for line in lines {
        let mut split_line = line.split(": ");
        let key = split_line.next().unwrap().trim();
        let value = split_line.next().unwrap_or("");

        if key == search_key {
            return Ok(value.to_string());
        }
    }

    Err(anyhow!("Could not find '{}' in pactl output", search_key))
}

fn print_attribute(t: &mut impl Write, key: &str, value: &str) {
    writeln!(t, "\x1b[1m{}: \x1b[0m{}", key, value).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_by_description() {
        let contents = fs::read_to_string("src/test/data/pactl-fiio.txt")
            .expect("Something went wrong reading the file");

        assert_eq!(
            find(&contents, "Description".to_string(), "Fiio", false).unwrap(),
            "43"
        )
    }

    #[test]
    fn test_matches_case_sensitive() {
        assert!(matches("test", "test", true))
    }

    #[test]
    fn test_matches_case_sensitive_with_capitals() {
        assert!(!matches("test", "Test", true))
    }

    #[test]
    fn test_matches_case_insensitive() {
        assert!(matches("Test", "test", false))
    }

    #[test]
    fn test_matches_case_insensitive_with_capitals() {
        assert!(matches("test", "Test", false))
    }

    #[test]
    fn test_print_attribute_emits_bold_key_plain_value() {
        let mut out = Vec::new();
        print_attribute(&mut out, "       Name", "alsa_output.usb-FiiO");
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "\x1b[1m       Name: \x1b[0malsa_output.usb-FiiO\n"
        );
    }

    #[test]
    fn test_matches_regex_metacharacters_literally() {
        // As a pattern, "(Game)" matched "Arctis 7 Game" and missed the real
        // device name; "Fiio +" failed to compile at all.
        assert!(matches("Arctis 7 (Game)", "(Game)", false));
        assert!(!matches("Arctis 7 Game", "(Game)", false));
        assert!(matches("Fiio + DAC", "Fiio +", false));
    }
}
