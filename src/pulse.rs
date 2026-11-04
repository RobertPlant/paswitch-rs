use crate::commands::run_pactl;
use anyhow::{anyhow, Result};
use std::io::{stdout, Write};

const LISTED_FIELDS: [&str; 6] = ["Description", "Name", "State", "Driver", "Mute", "Volume"];

/// The id from a `Sink #43` header, `None` for any other section.
fn parse_sink_id(header: &str) -> Option<&str> {
    let (section, id) = header.split_once(" #")?;
    let is_sink = section == "Sink";

    is_sink.then_some(id)
}

/// Splits a `Key: value` field or a `key.name = "value"` property, whichever separator comes first.
fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    let (at, separator) = [": ", " = "]
        .into_iter()
        .filter_map(|separator| line.find(separator).map(|at| (at, separator)))
        .min()?;
    let (key, rest) = line.split_at(at);
    let value = rest.strip_prefix(separator)?.trim_matches('"');

    Some((key.trim(), value))
}

/// Literal match, not a regex: device names are full of brackets, like "Arctis 7 (Game)".
fn has_needle(haystack: &str, needle: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        haystack.contains(needle)
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

pub fn search_sinks(search_key: &str, search_value: &str, case_sensitive: bool) -> Result<String> {
    find_sink(
        &run_pactl(&["list"])?,
        search_key,
        search_value,
        case_sensitive,
    )
    .ok_or_else(|| anyhow!("Search failed: no matching sink found"))
}

fn find_sink(output: &str, search_key: &str, needle: &str, case_sensitive: bool) -> Option<String> {
    output
        .split_terminator("\n\n")
        .find_map(|group| find_sink_in_group(group, search_key, needle, case_sensitive))
}

fn find_sink_in_group(
    group: &str,
    search_key: &str,
    needle: &str,
    case_sensitive: bool,
) -> Option<String> {
    let mut lines = group.lines();
    let id = lines.next().and_then(parse_sink_id)?;
    let is_match = lines
        .filter_map(parse_key_value)
        .any(|(key, value)| key == search_key && has_needle(value, needle, case_sensitive));

    is_match.then(|| id.to_string())
}

pub fn list_sinks() -> Result<()> {
    let output = run_pactl(&["list"])?;
    let listing = format_sinks(&output)?;

    stdout().write_all(listing.as_bytes())?;

    Ok(())
}

fn format_sinks(output: &str) -> Result<String> {
    let sinks = output
        .split_terminator("\n\n")
        .filter_map(|group| Some((group, group.lines().next().and_then(parse_sink_id)?)))
        .map(|(group, id)| format_sink(group, id))
        .collect::<Result<String>>()?;

    Ok(format!("\n{}", sinks))
}

fn format_sink(group: &str, id: &str) -> Result<String> {
    let fields = LISTED_FIELDS
        .iter()
        .map(|key| Ok(format_attribute(key, get_field(group, key)?)))
        .collect::<Result<String>>()?;

    Ok(format!("{}{}\n", format_attribute("ID", id), fields))
}

fn get_field<'a>(group: &'a str, search_key: &str) -> Result<&'a str> {
    group
        .lines()
        .filter_map(parse_key_value)
        .find(|(key, _)| *key == search_key)
        .map(|(_, value)| value)
        .ok_or_else(|| anyhow!("Could not find '{}' in pactl output", search_key))
}

fn format_attribute(key: &str, value: &str) -> String {
    format!("\x1b[1m{:>11}: \x1b[0m{}\n", key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIIO: &str = include_str!("test/data/pactl-fiio.txt");
    const FULL_LIST: &str = include_str!("test/data/pactl-list.txt");

    #[test]
    fn test_find_by_description() {
        assert_eq!(
            find_sink_in_group(FIIO, "Description", "Fiio", false),
            Some("43".to_string())
        )
    }

    #[test]
    fn test_find_ignores_matches_in_other_fields() {
        assert_eq!(
            find_sink_in_group(FIIO, "Name", "Analog Stereo", false),
            None
        )
    }

    #[test]
    fn test_find_by_property() {
        assert_eq!(
            find_sink_in_group(FIIO, "device.description", "Fiio", false),
            Some("43".to_string())
        )
    }

    #[test]
    fn test_find_sink_skips_other_sections() {
        assert_eq!(
            find_sink(FULL_LIST, "Description", "Navi 10", false),
            Some("45".to_string())
        )
    }

    #[test]
    fn test_find_sink_ignores_matching_sources() {
        assert_eq!(
            find_sink(FULL_LIST, "Description", "Monitor of", false),
            None
        )
    }

    #[test]
    fn test_find_sink_survives_trailing_blank_lines() {
        assert_eq!(
            find_sink("Sink #43\n\tName: a\n\n\n", "Name", "b", false),
            None
        )
    }

    #[test]
    fn test_get_field_reads_a_field_from_the_group() {
        assert_eq!(get_field(FIIO, "State").unwrap(), "RUNNING")
    }

    #[test]
    fn test_format_sinks_lists_each_sink() {
        assert_eq!(
            format_sinks(FULL_LIST).unwrap(),
            include_str!("test/data/pactl-list-formatted.txt")
        )
    }

    #[test]
    fn test_format_sinks_needs_every_listed_field() {
        assert!(format_sinks("Sink #43\n\tName: a\n").is_err())
    }

    #[test]
    fn test_parse_sink_id_without_a_header() {
        assert_eq!(parse_sink_id(""), None)
    }

    #[test]
    fn test_parse_key_value_keeps_colons_in_the_value() {
        assert_eq!(
            parse_key_value("\tDescription: Fiio: E10"),
            Some(("Description", "Fiio: E10"))
        )
    }

    #[test]
    fn test_parse_key_value_reads_properties() {
        assert_eq!(
            parse_key_value("\t\tdevice.description = \"Fiio: E10\""),
            Some(("device.description", "Fiio: E10"))
        )
    }

    #[test]
    fn test_parse_key_value_skips_section_titles() {
        assert_eq!(parse_key_value("\tProperties:"), None)
    }

    #[test]
    fn test_has_needle_case_sensitive() {
        assert!(has_needle("test", "test", true))
    }

    #[test]
    fn test_has_needle_case_sensitive_with_capitals() {
        assert!(!has_needle("test", "Test", true))
    }

    #[test]
    fn test_has_needle_case_insensitive() {
        assert!(has_needle("Test", "test", false))
    }

    #[test]
    fn test_has_needle_case_insensitive_with_capitals() {
        assert!(has_needle("test", "Test", false))
    }

    #[test]
    fn test_format_attribute_emits_bold_key_plain_value() {
        assert_eq!(
            format_attribute("Name", "alsa_output.usb-FiiO"),
            "\x1b[1m       Name: \x1b[0malsa_output.usb-FiiO\n"
        )
    }

    #[test]
    fn test_has_needle_matches_brackets_literally() {
        assert!(has_needle("Arctis 7 (Game)", "(Game)", false))
    }

    #[test]
    fn test_has_needle_does_not_treat_brackets_as_a_group() {
        assert!(!has_needle("Arctis 7 Game", "(Game)", false))
    }

    #[test]
    fn test_has_needle_matches_a_plus_literally() {
        assert!(has_needle("Fiio + DAC", "Fiio +", false))
    }
}
