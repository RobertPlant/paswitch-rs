use anyhow::{anyhow, Context, Result};
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

pub fn pactl(args: &[&str]) -> Result<String> {
    // pactl translates its labels ("Sink #" is "Ziel #" in German), the parser needs English
    let output = Command::new(PACTL)
        .env("LC_ALL", "C")
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute `{} {}`", PACTL, args.join(" ")))?;

    if !output.status.success() {
        return Err(anyhow!(
            "`{} {}` failed: {}",
            PACTL,
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8(output.stdout)?)
}
