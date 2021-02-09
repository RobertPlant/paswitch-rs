use std::process::Command;
use std::{error::Error, fmt};

use crate::commands::Type;

#[derive(Debug)]
pub struct PaswitchError {
    message: String,
}

impl Error for PaswitchError {}

impl fmt::Display for PaswitchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Paswitch failed: {}", &self.message)
    }
}

pub fn set_source(source: String) -> Result<(), PaswitchError> {
    let output = Command::new(Type::Paswitch.to_string())
        .arg(&source)
        .output()
        .expect("An invalid Sink has been supplied");

    if !output.status.success() {
        return Err(PaswitchError {
            message: "Paswitch failed to set source".to_string(),
        });
    }

    println!("Set pulse output to id: {}", source);

    Ok(())
}
