mod commands;
mod interactive;
mod paswitch;
mod pulse;
mod types;

use anyhow::Result;
use clap::{ArgGroup, Parser};
use interactive::prompt_for_sink;
use paswitch::set_source;
use pulse::{list_sinks, search_sinks};
use types::Type;

#[derive(Debug, Parser)]
#[command(version, arg_required_else_help = true)]
#[command(group(ArgGroup::new("mode").required(true).args(["search", "list", "interactive"])))]
struct Cli {
    /// Device to search for
    #[arg(value_parser = clap::builder::NonEmptyStringValueParser::new())]
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

fn main() -> Result<()> {
    let args = Cli::parse();
    let mode = Type::try_from(&args)?;

    match mode {
        Type::List => list_sinks()?,
        Type::Interactive => prompt_for_sink()?,
        Type::Set(needle) => set_source(&search_sinks(
            &args.search_key,
            &needle,
            args.case_sensitive,
        )?)?,
    }

    Ok(())
}
