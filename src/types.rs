use anyhow::{anyhow, Error};

use crate::Cli;

#[derive(Debug, PartialEq)]
pub enum Type {
    Interactive,
    List,
    Set(String),
}

impl TryFrom<&Cli> for Type {
    type Error = Error;

    fn try_from(args: &Cli) -> Result<Self, Self::Error> {
        match (&args.search, args.list, args.interactive) {
            (Some(search), false, false) => Ok(Type::Set(search.to_string())),
            (None, true, false) => Ok(Type::List),
            (None, false, true) => Ok(Type::Interactive),
            _ => Err(anyhow!("Give one of a search, --list or --interactive")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse_type(args: &[&str]) -> Option<Type> {
        let cli = Cli::try_parse_from([&["paswitch-rs"], args].concat()).ok()?;

        Type::try_from(&cli).ok()
    }

    #[test]
    fn test_parses_to_list() {
        assert_eq!(parse_type(&["--list"]), Some(Type::List))
    }

    #[test]
    fn test_parses_to_interactive() {
        assert_eq!(parse_type(&["--interactive"]), Some(Type::Interactive))
    }

    #[test]
    fn test_parses_to_set() {
        assert_eq!(parse_type(&["Fiio"]), Some(Type::Set("Fiio".to_string())))
    }

    #[test]
    fn test_rejects_no_mode() {
        assert_eq!(parse_type(&["--case-sensitive"]), None)
    }

    #[test]
    fn test_rejects_two_modes() {
        assert_eq!(parse_type(&["--list", "--interactive"]), None)
    }

    #[test]
    fn test_rejects_a_search_with_a_mode() {
        assert_eq!(parse_type(&["--list", "Fiio"]), None)
    }

    #[test]
    fn test_rejects_an_empty_search() {
        assert_eq!(parse_type(&[""]), None)
    }
}
