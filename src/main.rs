extern crate term;

mod paswitch;
mod pulse;

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Device to search for
    #[structopt(required_unless("list"), default_value = "")]
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

    if args.list {
        pulse::list();
    } else {
        let response = match pulse::search(args.search_key, args.search, args.case_sensitive) {
            Ok(id) => paswitch::set_source(id).unwrap(),
            Err(err) => err,
        };
        println!("{}", response);
    }

    Ok(())
}
