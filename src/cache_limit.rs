use command::{Command, Query};
use command::Command::CacheLimit;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CacheLimitCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for CacheLimitCommand {
    fn default() -> CacheLimitCommand {
        CacheLimitCommand {
            command: CacheLimit,
            arguments: HashMap::new(),
        }
    }
}

impl CacheLimitCommand {
    pub fn new() -> CacheLimitCommand {
        CacheLimitCommand::default()
    }

    pub fn max(mut self, max: u64) -> CacheLimitCommand {
        let string = format!("{}", max);
        self.arguments.insert("max".to_string(), string.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (CacheLimit, query)
    }
}

impl Queryable for CacheLimitCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for CacheLimitCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(CacheLimitCommand);
request_cancellable!(CacheLimitCommand);
request_timeoutable!(CacheLimitCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::CacheLimit;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let cache_limit = CacheLimitCommand::new();
        let expected = CacheLimitCommand {
            command: CacheLimit,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, cache_limit);
    }

    #[test]
    fn test_max() {
        let cache_limit = CacheLimitCommand::new().max(4);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "4".to_string());
        let expected = CacheLimitCommand {
            command: CacheLimit,
            arguments: arg.to_owned(),
        };
        assert_eq!(expected, cache_limit);
    }

    #[test]
    fn test_build() {
        let actual = CacheLimitCommand::new().max(2).build();
        let expected_query: Query = vec![("max".to_string(), "2".to_string())];
        let expected = (CacheLimit, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = CacheLimitCommand::new().max(40).to_query();
        let url_encoded = "/d/cache_limit?max=40";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = CacheLimitCommand::new().max(80).to_command();
        let cli_encoded = "cache_limit --max 80";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = CacheLimitCommand {
            command: CacheLimit,
            arguments: arg.to_owned(),
        };
        let query = CacheLimitCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
