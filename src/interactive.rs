use std::{error::Error, fmt};

use crate::paswitch::set_source;
use crate::pulse::list;

#[derive(Debug)]
pub enum CursesErrorType {
    PulseSubCommandFailed,
}

#[derive(Debug)]
pub struct CursesError {
    error_type: CursesErrorType,
    message: String,
}

impl Error for CursesError {}

impl fmt::Display for CursesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your curses request failed: {}", &self.message)
    }
}

pub fn interactive() -> Result<(), CursesError> {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    match list() {
        Ok(_) => {}
        Err(_) => {
            return Err(CursesError {
                error_type: CursesErrorType::PulseSubCommandFailed,
                message: "Listing failed".to_owned(),
            })
        }
    }

    print!("Please select an input id: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    match set_source(s) {
        Ok(_) => {}
        Err(_) => {
            return Err(CursesError {
                error_type: CursesErrorType::PulseSubCommandFailed,
                message: "Setting failed".to_owned(),
            })
        }
    }

    Ok(())
}
