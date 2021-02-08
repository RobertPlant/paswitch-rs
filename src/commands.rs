use std::process::Command;
use std::{error::Error, fmt};
use strum_macros::ToString;

#[derive(Debug)]
pub enum CommandErrorType {
    Missing,
}

#[derive(Debug)]
pub struct CommandError {
    error_type: CommandErrorType,
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

#[derive(ToString, Debug)]
pub enum Type {
    #[strum(serialize = "paswitch")]
    Paswitch,

    #[strum(serialize = "pactl")]
    Pactl,
}

pub fn check_command(command: Type) -> Result<(), CommandError> {
    match Command::new(command.to_string()).output() {
        Ok(_) => Ok(()),
        _ => Err(CommandError {
            error_type: CommandErrorType::Missing,
            command: command,
        }),
    }
}
