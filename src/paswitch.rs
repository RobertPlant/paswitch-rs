use anyhow::{Context, Result};

use crate::commands::run_pactl;

/// First tab-separated field of each `pactl list short sink-inputs` line.
fn parse_sink_input_ids(list_output: &str) -> Vec<&str> {
    list_output
        .lines()
        .filter_map(|line| line.split('\t').next())
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .collect()
}

pub fn set_source(source: &str) -> Result<()> {
    run_pactl(&["set-default-sink", source])
        .context("Failed to set the default sink - invalid sink may have been supplied")?;

    // Setting the default only affects new streams; anything already playing
    // stays on the old sink until it is moved explicitly.
    for id in parse_sink_input_ids(&run_pactl(&["list", "short", "sink-inputs"])?) {
        // A stream can end between the listing and the move, so a failure here
        // is expected and must not abort a switch that has already succeeded.
        let _ = run_pactl(&["move-sink-input", id, source]);
    }

    println!("Set pulse output to id: {}", source);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sink_input_ids() {
        let output = "5\t44\t1000\tPipeWire\ts16le 2ch 48000Hz\n\
                      12\t44\t1000\tPipeWire\tfloat32le 2ch 48000Hz\n";

        assert_eq!(parse_sink_input_ids(output), vec!["5", "12"])
    }

    #[test]
    fn test_parse_sink_input_ids_when_nothing_is_playing() {
        assert!(parse_sink_input_ids("").is_empty());
        assert!(parse_sink_input_ids("\n").is_empty())
    }
}
