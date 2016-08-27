use command::{Command, Query};
use command::Command::Tokenize;
use std::collections::HashMap;
use types::tokenizer_type::TokenizerType;
use types::token_filters_type::TokenFiltersType;
use types::normalizer_type::NormalizerType;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;
use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::FlagType::{None, EnableTokenizedDelimiter, ExtFlagType};
use self::ModeType::{Add, Get, ExtModeType};
use util;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TokenizeCommand {
    command: Command,
    tokenizer: TokenizerType,
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
    EnableTokenizedDelimiter,
    /// For future extensibility.
    ExtFlagType(String),
}

impl AsRef<str> for FlagType {
    fn as_ref(&self) -> &str {
        match *self {
            None => "NONE",
            EnableTokenizedDelimiter => "ENABLE_TOKENIZED_DELIMITER",
            ExtFlagType(ref s) => s.as_ref(),
        }
    }
}

impl fmt::Display for FlagType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            None => "NONE",
            EnableTokenizedDelimiter => "ENABLE_TOKENIZED_DELIMITER",
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
                "EnableTokenDelimiter" |
                "ENABLE_TOKENIZED_DELIMITER" => EnableTokenizedDelimiter,
                _ => ExtFlagType(s.to_owned()),
            })
        }
    }
}

#[derive (Debug)]
pub enum ModeTypeError {
    Empty,
}

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum ModeType {
    Add,
    Get,
    /// For future extensibility.
    ExtModeType(String),
}

impl AsRef<str> for ModeType {
    fn as_ref(&self) -> &str {
        match *self {
            Add => "ADD",
            Get => "GET",
            ExtModeType(ref s) => s.as_ref(),
        }
    }
}

impl fmt::Display for ModeType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Add => "ADD",
            Get => "GET",
            ExtModeType(ref s) => s.as_ref(),
        })
    }
}

impl FromStr for ModeType {
    type Err = ModeTypeError;
    fn from_str(s: &str) -> Result<ModeType, ModeTypeError> {
        if s == "" {
            Err(ModeTypeError::Empty)
        } else {
            Ok(match s {
                "Add" | "ADD" => Add,
                "Get" | "GET" => Get,
                _ => ExtModeType(s.to_owned()),
            })
        }
    }
}

impl Default for TokenizeCommand {
    fn default() -> TokenizeCommand {
        TokenizeCommand {
            command: Tokenize,
            tokenizer: TokenizerType::Bigram,
            string: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl TokenizeCommand {
    pub fn new(tokenizer: TokenizerType, string: String) -> TokenizeCommand {
        let default: TokenizeCommand = Default::default();
        TokenizeCommand {
            command: default.command,
            tokenizer: tokenizer,
            string: format!("\"{}\"", string),
            arguments: default.arguments,
        }
    }

    pub fn normalizer(mut self, normalizer_type: NormalizerType) -> TokenizeCommand {
        let string = format!("{}", normalizer_type);
        self.arguments.insert("normalizer".to_string(), string.to_owned());
        self
    }

    pub fn token_filters(mut self, token_filters: TokenFiltersType) -> TokenizeCommand {

        let string = format!("{}", token_filters);
        self.arguments.insert("token_filters".to_string(), string.to_owned());
        self
    }

    pub fn flags(mut self, flags: Vec<FlagType>) -> TokenizeCommand {
        let string = util::split_flags_vec(flags);
        self.arguments.insert("flags".to_string(), string.to_owned());
        self
    }

    pub fn mode(mut self, mode: ModeType) -> TokenizeCommand {
        let string = format!("{}", mode);
        self.arguments.insert("mode".to_string(), string.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("tokenizer".to_string(), format!("{}", self.tokenizer)),
                                    ("string".to_string(), self.string)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Tokenize, query)
    }
}

impl Queryable for TokenizeCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for TokenizeCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(TokenizeCommand);
request_cancellable!(TokenizeCommand);
request_timeoutable!(TokenizeCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Tokenize;
    use std::collections::HashMap;
    use types::token_filters_type::TokenFiltersType;
    use types::tokenizer_type::TokenizerType;
    use types::normalizer_type::NormalizerType;
    use std::str::FromStr;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_flag_type_from_str() {
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
    fn test_flag_type_fmt() {
        assert_eq!("NONE".to_owned(), format!("{}", FlagType::None));
        assert_eq!("ENABLE_TOKENIZED_DELIMITER".to_owned(),
                   format!("{}", FlagType::EnableTokenizedDelimiter));
        assert_eq!("added-type".to_owned(),
                   format!("{}", FlagType::ExtFlagType("added-type".to_owned())));
    }

    #[test]
    fn test_flag_type_as_str() {
        assert_eq!(FlagType::None.as_ref(), "NONE");
        assert_eq!(FlagType::EnableTokenizedDelimiter.as_ref(),
                   "ENABLE_TOKENIZED_DELIMITER");
        assert_eq!(FlagType::ExtFlagType("added-type".to_owned()).as_ref(),
                   "added-type");
    }

    #[test]
    fn test_mode_from_str() {
        assert_eq!(ModeType::Add, FromStr::from_str("Add").unwrap());
        assert_eq!(ModeType::ExtModeType("added-type".to_owned()),
                   FromStr::from_str("added-type").unwrap());
        let x: Result<ModeType, _> = FromStr::from_str("");
        if let Err(ModeTypeError::Empty) = x {
        } else {
            panic!("An empty input type is invalid!")
        }
    }

    #[test]
    fn test_mode_fmt() {
        assert_eq!("ADD".to_owned(), format!("{}", ModeType::Add));
        assert_eq!("GET".to_owned(), format!("{}", ModeType::Get));
        assert_eq!("added-type".to_owned(),
                   format!("{}", FlagType::ExtFlagType("added-type".to_owned())));
    }

    #[test]
    fn test_mode_as_str() {
        assert_eq!(ModeType::Add.as_ref(), "ADD");
        assert_eq!(ModeType::Get.as_ref(), "GET");
        assert_eq!(FlagType::ExtFlagType("added-type".to_owned()).as_ref(),
                   "added-type");
    }

    #[test]
    fn test_new() {
        let tokenize = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string());
        let expected = TokenizeCommand {
            command: Tokenize,
            tokenizer: TokenizerType::Bigram,
            string: "\"element\"".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, tokenize);
    }

    #[test]
    fn test_normalizer() {
        let column_create = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string())
            .normalizer(NormalizerType::Auto);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("normalizer".to_string(),
                   format!("{}", NormalizerType::Auto));
        let expected = TokenizeCommand {
            command: Tokenize,
            tokenizer: TokenizerType::Bigram,
            string: "\"element\"".to_string(),
            arguments: arg.to_owned(),
        };
        assert_eq!(expected, column_create);
    }

    #[test]
    fn test_token_filters() {
        let token_filters = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string())
            .token_filters(TokenFiltersType::StopWord);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("token_filters".to_string(),
                   format!("{}", TokenFiltersType::StopWord));
        let expected = TokenizeCommand {
            command: Tokenize,
            tokenizer: TokenizerType::Bigram,
            string: "\"element\"".to_string(),
            arguments: arg.to_owned(),
        };
        assert_eq!(expected, token_filters);
    }

    #[test]
    fn test_flags() {
        let tokenize = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string())
            .flags(vec![(FlagType::None), (FlagType::EnableTokenizedDelimiter)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("flags".to_string(),
                   "NONE|ENABLE_TOKENIZED_DELIMITER".to_string());
        let expected = TokenizeCommand {
            command: Tokenize,
            tokenizer: TokenizerType::Bigram,
            string: "\"element\"".to_string(),
            arguments: arg.to_owned(),
        };
        assert_eq!(expected, tokenize);
    }

    #[test]
    fn test_mode() {
        let tokenize = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string())
            .mode(ModeType::Add);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("mode".to_string(), format!("{}", ModeType::Add));
        let expected = TokenizeCommand {
            command: Tokenize,
            tokenizer: TokenizerType::Bigram,
            string: "\"element\"".to_string(),
            arguments: arg.to_owned(),
        };
        assert_eq!(expected, tokenize);
    }

    #[test]
    fn test_build() {
        let actual = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string()).build();
        let expected_query: Query = vec![("tokenizer".to_string(),
                                          format!("{}", TokenizerType::Bigram)),
                                         ("string".to_string(), "\"element\"".to_string())];
        let expected = (Tokenize, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string()).to_query();
        let url_encoded = "/d/tokenize?tokenizer=TokenBigram&string=%22element%22";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string()).to_command();
        let cli_encoded = "tokenize --tokenizer TokenBigram --string \"element\"";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = TokenizeCommand {
            command: Tokenize,
            tokenizer: TokenizerType::Bigram,
            string: "\"element\"".to_string(),
            arguments: arg.to_owned(),
        };
        let query = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
