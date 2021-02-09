use regex::{Regex, RegexBuilder};
use std::io::prelude::Write;
use std::process::Command;
use std::str::FromStr;
use std::str::Lines;
use std::{error::Error, fmt};
use term;
use term::StdoutTerminal;

use crate::commands::Type;

#[derive(Debug, PartialEq)]
pub enum EntityType {
    Sink,
    Module,
    Source,
    Input,
    SinkInput,
    Client,
    Card,
}

impl FromStr for EntityType {
    type Err = PulseError;

    fn from_str(input: &str) -> Result<EntityType, Self::Err> {
        match input {
            "Sink" => Ok(EntityType::Sink),
            "Module" => Ok(EntityType::Module),
            "Source" => Ok(EntityType::Source),
            "Input" => Ok(EntityType::Input),
            "Sink Input" => Ok(EntityType::SinkInput),
            "Client" => Ok(EntityType::Client),
            "Card" => Ok(EntityType::Card),
            _ => Err(PulseError {
                error_type: PulseErrorType::UnknownEntityType,
                message: format!("Could not map entity with type {}", input),
            }),
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

#[derive(Debug)]
pub enum PulseErrorType {
    NotFound,
    NotSink,
    UnknownEntityType,
}

#[derive(Debug)]
pub struct PulseError {
    error_type: PulseErrorType,
    message: String,
}

impl Error for PulseError {}

impl fmt::Display for PulseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your pulse request failed: {}", &self.message)
    }
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

fn get_search_pattern(search_value: String, case_sensitive: bool) -> Result<Regex, regex::Error> {
    RegexBuilder::new(&format!(".*{}.*", search_value).to_owned())
        .case_insensitive(!case_sensitive)
        .build()
}

pub fn search(
    search_key: String,
    search_value: String,
    case_sensitive: bool,
) -> Result<String, PulseError> {
    let pattern = get_search_pattern(search_value, case_sensitive)
        .expect("Something went wrong building the regex");

    for group in list_sinks().split_terminator("\n\n") {
        match find(group, search_key.to_owned(), pattern.to_owned()) {
            Ok(id) => return Ok(id),
            _ => continue,
        }
    }

    Err(PulseError {
        error_type: PulseErrorType::NotFound,
        message: "Search failed".to_owned(),
    })
}

fn find(group: &str, search_key: String, pattern: Regex) -> Result<String, PulseError> {
    let mut lines = group.lines();
    let mut first_line = lines.next().unwrap().split(" #");
    let group_type = EntityType::from_str(first_line.next().unwrap())?;
    let id = String::from(first_line.next().unwrap());

    if group_type != EntityType::Sink {
        return Err(PulseError {
            error_type: PulseErrorType::NotSink,
            message: "Not a Sink".to_owned(),
        });
    }

    for line in lines {
        let mut split_line = line.split(": ");
        let key = split_line.next().unwrap().trim();
        let value = split_line.next().unwrap_or("");

        if key == search_key && pattern.is_match(value) {
            return Ok(id);
        }
    }

    Err(PulseError {
        error_type: PulseErrorType::NotFound,
        message: "Not matched".to_owned(),
    })
}

pub fn list() -> Result<(), PulseError> {
    let mut t = term::stdout().unwrap();
    write!(t, "\n").unwrap();

    for group in list_sinks().split_terminator("\n\n") {
        let mut lines = group.lines();
        let mut first_line = lines.next().unwrap().split(" #");
        let group_type = EntityType::from_str(first_line.next().unwrap())?;
        let id = String::from(first_line.next().unwrap());

        if group_type != EntityType::Sink {
            continue;
        }

        let sink = Entity {
            id: id,
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
        write!(t, "\n").unwrap();
    }

    Ok(())
}

fn pull_data(lines: &mut Lines, search_key: String) -> Result<String, PulseError> {
    for line in lines {
        let mut split_line = line.split(": ");
        let key = split_line.next().unwrap().trim();
        let value = split_line.next().unwrap_or("");

        if key == search_key {
            return Ok(value.to_string());
        }
    }

    Err(PulseError {
        error_type: PulseErrorType::NotFound,
        message: "Could not find data from pactl".to_owned(),
    })
}

fn print_attribute(t: &mut Box<StdoutTerminal>, key: &str, value: &str) {
    t.attr(term::Attr::Bold).unwrap();
    write!(t, "{}: ", key).unwrap();
    t.reset().unwrap();
    write!(t, "{}\n", value).unwrap();
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
