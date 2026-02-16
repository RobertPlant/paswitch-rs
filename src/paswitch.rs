use anyhow::{anyhow, Context, Result};
use std::process::Command;

use crate::commands::Type;

pub fn set_source(source: String) -> Result<()> {
    let output = Command::new(Type::Paswitch.to_string())
        .arg(&source)
        .output()
        .context("Failed to execute paswitch - invalid sink may have been supplied")?;

    if !output.status.success() {
        return Err(anyhow!("Paswitch failed to set source"));
    }

    println!("Set pulse output to id: {}", source);

    Ok(())
}
