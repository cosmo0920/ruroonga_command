use command::{Command, Query};
use command::Command::Normalize;
use std::collections::HashMap;
use types::normalizer_type::NormalizerType;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;
use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::FlagType::{None, RemoveBlank, WithTypes, WithChecks, RemoveTokenizedDelimiter,
                     ExtFlagType};
use util;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NormalizeCommand {
    command: Command,
    normalizer: NormalizerType,
    string: String,
    arguments: HashMap<String, String>,
}

#[derive (Debug)]
pub enum FlagTypeError {
    Empty,
}

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum FlagType {
    None,
    RemoveBlank,
    WithTypes,
    WithChecks,
    RemoveTokenizedDelimiter,
    /// For future extensibility.
    ExtFlagType(String),
}

impl AsRef<str> for FlagType {
    fn as_ref(&self) -> &str {
        match *self {
            None => "NONE",
            RemoveBlank => "REMOVE_BLANK",
            WithTypes => "WITH_TYPES",
            WithChecks => "WITH_CHECKS",
            RemoveTokenizedDelimiter => "REMOVE_TOKENIZED_DELIMITER",
            ExtFlagType(ref s) => s.as_ref(),
        }
    }
}

impl fmt::Display for FlagType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            None => "NONE",
            RemoveBlank => "REMOVE_BLANK",
            WithTypes => "WITH_TYPES",
            WithChecks => "WITH_CHECKS",
            RemoveTokenizedDelimiter => "REMOVE_TOKENIZED_DELIMITER",
            ExtFlagType(ref s) => s.as_ref(),
        })
    }
}

impl FromStr for FlagType {
    type Err = FlagTypeError;
    fn from_str(s: &str) -> Result<FlagType, FlagTypeError> {
        if s == "" {
            Err(FlagTypeError::Empty)
        } else {
            Ok(match s {
                "None" | "NONE" => None,
                "RemoveBlank" | "REMOVE_BLANK" => RemoveBlank,
                "WithTypes" | "WITH_TYPES" => WithTypes,
                "WithChecks" | "WITH_CHECKS" => WithChecks,
                "RemoveTokenizedDelimiter" |
                "REMOVE_TOKENIZED_DELIMITER" => RemoveTokenizedDelimiter,
                _ => ExtFlagType(s.to_owned()),
            })
        }
    }
}

impl Default for NormalizeCommand {
    fn default() -> NormalizeCommand {
        NormalizeCommand {
            command: Normalize,
            normalizer: NormalizerType::Auto,
            string: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl NormalizeCommand {
    pub fn new(normalizer: NormalizerType, string: String) -> NormalizeCommand {
        NormalizeCommand {
            normalizer: normalizer,
            string: format!("\"{}\"", string),
            ..NormalizeCommand::default()
        }
    }

    pub fn flags(mut self, flags: Vec<FlagType>) -> NormalizeCommand {
        let string = util::split_flags_vec(flags);
        self.arguments.insert("flags".to_string(), string.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("normalizer".to_string(), format!("{}", self.normalizer)),
                                    ("string".to_string(), self.string)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (self.command, query)
    }
}

impl Queryable for NormalizeCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for NormalizeCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

request_cancellable!(NormalizeCommand);
request_timeoutable!(NormalizeCommand);

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::str::FromStr;
    use types::normalizer_type::NormalizerType;
    use command::Command::Normalize;
    use queryable::Queryable;
    use commandable::Commandable;

    #[test]
    fn test_from_str() {
        assert_eq!(FlagType::None, FromStr::from_str("None").unwrap());
        assert_eq!(FlagType::ExtFlagType("added-type".to_owned()),
                   FromStr::from_str("added-type").unwrap());
        let x: Result<FlagType, _> = FromStr::from_str("");
        if let Err(FlagTypeError::Empty) = x {
        } else {
            panic!("An empty input type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("NONE".to_owned(), format!("{}", FlagType::None));
        assert_eq!("REMOVE_BLANK".to_owned(),
                   format!("{}", FlagType::RemoveBlank));
        assert_eq!("WITH_TYPES".to_owned(), format!("{}", FlagType::WithTypes));
        assert_eq!("WITH_CHECKS".to_owned(),
                   format!("{}", FlagType::WithChecks));
        assert_eq!("REMOVE_TOKENIZED_DELIMITER".to_owned(),
                   format!("{}", FlagType::RemoveTokenizedDelimiter));
        assert_eq!("added-type".to_owned(),
                   format!("{}", FlagType::ExtFlagType("added-type".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(FlagType::None.as_ref(), "NONE");
        assert_eq!(FlagType::RemoveBlank.as_ref(), "REMOVE_BLANK");
        assert_eq!(FlagType::WithTypes.as_ref(), "WITH_TYPES");
        assert_eq!(FlagType::WithChecks.as_ref(), "WITH_CHECKS");
        assert_eq!(FlagType::RemoveTokenizedDelimiter.as_ref(),
                   "REMOVE_TOKENIZED_DELIMITER");
        assert_eq!(FlagType::ExtFlagType("added-type".to_owned()).as_ref(),
                   "added-type");
    }

    #[test]
    fn test_new() {
        let normalize = NormalizeCommand::new(NormalizerType::Auto, "element".to_string());
        let expected = NormalizeCommand {
            command: Normalize,
            normalizer: NormalizerType::Auto,
            string: "\"element\"".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, normalize);
    }

    #[test]
    fn test_flags() {
        let normalize = NormalizeCommand::new(NormalizerType::Auto, "element".to_string())
            .flags(vec![(FlagType::RemoveBlank), (FlagType::WithTypes)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("flags".to_string(), "REMOVE_BLANK|WITH_TYPES".to_string());
        let expected = NormalizeCommand {
            command: Normalize,
            normalizer: NormalizerType::Auto,
            string: "\"element\"".to_string(),
            arguments: arg.to_owned(),
        };
        assert_eq!(expected, normalize);
    }

    #[test]
    fn test_build() {
        let actual = NormalizeCommand::new(NormalizerType::Auto, "element".to_string()).build();
        let expected = (Normalize,
                        vec![("normalizer".to_string(), format!("{}", NormalizerType::Auto)),
                             ("string".to_string(), "\"element\"".to_string())]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = NormalizeCommand::new(NormalizerType::Auto, "element".to_string()).to_query();
        let url_encoded = "/d/normalize?normalizer=NormalizerAuto&string=%22element%22";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = NormalizeCommand::new(NormalizerType::Auto, "element".to_string()).to_command();
        let cli_encoded = "normalize --normalizer NormalizerAuto --string \"element\"";
        assert_eq!(cli_encoded.to_string(), query);
    }
}
