use anyhow::{anyhow, Result};
use std::process::Command;
use strum_macros::Display;

#[derive(Display, Debug)]
pub enum Type {
    #[strum(serialize = "paswitch")]
    Paswitch,

    #[strum(serialize = "pactl")]
    Pactl,
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
