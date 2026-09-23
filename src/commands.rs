use anyhow::{anyhow, Result};
use std::process::Command;

pub const PACTL: &str = "pactl";

pub fn check_command(command: &str) -> Result<()> {
    match Command::new(command).output() {
        Ok(_) => Ok(()),
        _ => Err(anyhow!(
            "You need to install `{}` and add it to your path",
            command
        )),
    }
}
