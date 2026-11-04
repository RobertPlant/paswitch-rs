use anyhow::{anyhow, Context, Result};
use std::io::ErrorKind;
use std::process::Command;

pub const PACTL: &str = "pactl";

// pactl translates its labels ("Sink #" is "Ziel #" in German), the parser needs English
fn build_pactl(args: &[&str]) -> Command {
    let mut command = Command::new(PACTL);
    command.env("LC_ALL", "C").args(args);

    command
}

pub fn run_pactl(args: &[&str]) -> Result<String> {
    let output = match build_pactl(args).output() {
        Ok(output) => output,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            return Err(anyhow!(
                "You need to install `{}` and add it to your path",
                PACTL
            ))
        }
        Err(err) => {
            return Err(err)
                .with_context(|| format!("Failed to execute `{} {}`", PACTL, args.join(" ")))
        }
    };

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_build_pactl_asks_for_english_labels() {
        assert_eq!(
            build_pactl(&["list"]).get_envs().collect::<Vec<_>>(),
            vec![(OsStr::new("LC_ALL"), Some(OsStr::new("C")))]
        )
    }
}
