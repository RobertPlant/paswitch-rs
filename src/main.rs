extern crate term;

mod commands;
mod paswitch;
mod pulse;

use quicli::prelude::CliResult;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Device to search for
    #[structopt(required_unless("list"))]
    search: String,

    /// The field from `pactl list` that should be searched
    #[structopt(short, long, default_value = "Description")]
    search_key: String,

    /// Should the search be case sensitive
    #[structopt(short, long)]
    case_sensitive: bool,

    /// List available pulse sinks
    #[structopt(short, long)]
    list: bool,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    commands::check_command(commands::Type::Paswitch)?;

    if args.list {
        return Ok(pulse::list()?);
    } else if args.search.chars().count() > 0 {
        commands::check_command(commands::Type::Pactl)?;
        let id = pulse::search(args.search_key, args.search, args.case_sensitive)?;

        return Ok(paswitch::set_source(id)?);
    }

    Ok(())
}
