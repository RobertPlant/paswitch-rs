mod paswitch;
mod pulse;

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    search: String,

    #[structopt(short = "s", default_value = "Description")]
    key: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    let response = match pulse::search((args.key, args.search)) {
        Ok(id) => paswitch::set_source(id).unwrap(),
        Err(err) => err,
    };

    println!("{}", response);

    Ok(())
}
