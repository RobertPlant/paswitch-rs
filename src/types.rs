use crate::Cli;

#[derive(Debug, PartialEq)]
pub enum Type {
    List,
    Set,
    Unknown,
}

impl From<&Cli> for Type {
    fn from(args: &Cli) -> Self {
        if args.list {
            Type::List
        } else if args.search.chars().count() > 0 {
            Type::Set
        } else {
            Type::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_to_list() {
        assert_eq!(
            Type::from(&Cli {
                list: true,
                case_sensitive: false,
                search: "".to_owned(),
                search_key: "".to_owned()
            }),
            Type::List
        )
    }

    #[test]
    fn test_preferrs_list() {
        assert_eq!(
            Type::from(&Cli {
                list: true,
                case_sensitive: false,
                search: "Fiio".to_owned(),
                search_key: "".to_owned()
            }),
            Type::List
        )
    }

    #[test]
    fn test_parses_to_set() {
        assert_eq!(
            Type::from(&Cli {
                list: false,
                case_sensitive: false,
                search: "Fiio".to_owned(),
                search_key: "".to_owned()
            }),
            Type::Set
        )
    }

    #[test]
    fn test_parses_to_unknown() {
        assert_eq!(
            Type::from(&Cli {
                list: false,
                case_sensitive: false,
                search: "".to_owned(),
                search_key: "".to_owned()
            }),
            Type::Unknown
        )
    }
}
