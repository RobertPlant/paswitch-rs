use std::process::Command;

use crate::commands::Type;

pub fn set_source(source: String) -> Result<String, String> {
    let output = Command::new(Type::Paswitch.to_string())
        .arg(&source)
        .output()
        .expect("An invalid Sink has been supplied");

    if !output.status.success() {
        println!("error");

        return Err("Error".to_owned());
    }

    Ok(format!("Set pulse output to id: {}", source))
}
