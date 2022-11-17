extern crate term;

mod commands;
mod interactive;
mod paswitch;
mod pulse;
mod types;

use clap::Parser;
use commands::{
    check_command,
    Type::{Pactl, Paswitch},
};
use exitfailure::ExitFailure;
use interactive::interactive;
use paswitch::set_source;
use pulse::{list, search};
use types::Type;

#[derive(Debug, Parser)]
struct Cli {
    /// Device to search for
    search: Option<String>,

    /// The field from `pactl list` that should be searched
    #[clap(short, long, default_value = "Description")]
    search_key: String,

    /// Should the search be case sensitive
    #[clap(short, long)]
    case_sensitive: bool,

    /// List available pulse sinks
    #[clap(short, long)]
    list: bool,

    /// Interactive device selection
    #[clap(short, long)]
    interactive: bool,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::parse();
    check_command(Paswitch)?;

    Ok(match Type::from(&args) {
        Type::List => list()?,
        Type::Interactive => interactive()?,
        Type::Set => {
            check_command(Pactl)?;

            set_source(search(
                args.search_key,
                args.search.unwrap(),
                args.case_sensitive,
            )?)?
        }
        _ => (),
    })
}
