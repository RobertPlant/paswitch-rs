extern crate term;

mod commands;
mod interactive;
mod paswitch;
mod pulse;
mod types;

use commands::{
    check_command,
    Type::{Pactl, Paswitch},
};
use interactive::interactive;
use paswitch::set_source;
use pulse::{list, search};
use quicli::prelude::CliResult;
use structopt::StructOpt;
use types::Type;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Device to search for
    #[structopt(required_unless("list"), required_unless("interactive"))]
    search: Option<String>,

    /// The field from `pactl list` that should be searched
    #[structopt(short, long, default_value = "Description")]
    search_key: String,

    /// Should the search be case sensitive
    #[structopt(short, long)]
    case_sensitive: bool,

    /// List available pulse sinks
    #[structopt(short, long)]
    list: bool,

    /// Interactive device selection
    #[structopt(short, long)]
    interactive: bool,
}

fn main() -> CliResult {
    let args = Cli::from_args();

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
