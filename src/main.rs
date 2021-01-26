mod paswitch;
mod pulse;

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Device to search for
    search: String,

    /// Should the search be case sensitive
    #[structopt(short, long)]
    case_sensitive: bool,

    /// The field from `pactl list` that should be searched
    #[structopt(short = "s", default_value = "Description")]
    search_key: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    let response = match pulse::search(args.search_key, args.search, args.case_sensitive) {
        Ok(id) => paswitch::set_source(id).unwrap(),
        Err(err) => err,
    };

    println!("{}", response);

    Ok(())
}
