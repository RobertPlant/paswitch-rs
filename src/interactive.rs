use anyhow::{Context, Result};
use std::io::{stdin, stdout, Write};

use crate::paswitch::set_source;
use crate::pulse::list_sinks;

pub fn prompt_for_sink() -> Result<()> {
    let mut s = String::new();

    list_sinks().context("Failed to list audio sinks")?;

    print!("Please select an input id: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).context("Failed to read input")?;

    set_source(s.trim_end()).context("Failed to set audio source")?;

    Ok(())
}
