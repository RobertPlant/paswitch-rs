use std::process::Command;
use std::{error::Error, fmt};
use strum_macros::Display;

#[derive(Debug)]
pub struct CommandError {
    command: Type,
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "You need to install `{}` and set it into your path",
            &self.command.to_string()
        )
    }
}

#[derive(Display, Debug)]
pub enum Type {
    #[strum(serialize = "paswitch")]
    Paswitch,

    #[strum(serialize = "pactl")]
    Pactl,
}

pub fn check_command(command: Type) -> Result<(), CommandError> {
    match Command::new(command.to_string()).output() {
        Ok(_) => Ok(()),
        _ => Err(CommandError { command }),
    }
}
