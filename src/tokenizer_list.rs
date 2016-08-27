use command::{Command, Query};
use command::Command::TokenizerList;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TokenizerListCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for TokenizerListCommand {
    fn default() -> TokenizerListCommand {
        TokenizerListCommand {
            command: TokenizerList,
            arguments: HashMap::new(),
        }
    }
}

impl TokenizerListCommand {
    pub fn new() -> TokenizerListCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (TokenizerList, query)
    }
}

impl Queryable for TokenizerListCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for TokenizerListCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(TokenizerListCommand);
request_cancellable!(TokenizerListCommand);
request_timeoutable!(TokenizerListCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::TokenizerList;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let tokenizer_list = TokenizerListCommand::new();
        let expected = TokenizerListCommand {
            command: TokenizerList,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, tokenizer_list);
    }

    #[test]
    fn test_build() {
        let actual = TokenizerListCommand::new().build();
        let expected_query: Query = vec![];
        let expected = (TokenizerList, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = TokenizerListCommand::new().to_query();
        let url_encoded = "/d/tokenizer_list?";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = TokenizerListCommand::new().to_command();
        let url_encoded = "tokenizer_list";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = TokenizerListCommand {
            command: TokenizerList,
            arguments: arg.to_owned(),
        };
        let query = TokenizerListCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
