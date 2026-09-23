use anyhow::{anyhow, Result};
use std::process::Command;

#[derive(Debug)]
pub enum Type {
    Paswitch,
    Pactl,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Type::Paswitch => "paswitch",
            Type::Pactl => "pactl",
        })
    }
}

pub fn check_command(command: Type) -> Result<()> {
    match Command::new(command.to_string()).output() {
        Ok(_) => Ok(()),
        _ => Err(anyhow!(
            "You need to install `{}` and add it to your path",
            command
        )),
    }
}
