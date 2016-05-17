use super::command::{Command, Query};
use super::command::Command::ThreadLimit;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ThreadLimitCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for ThreadLimitCommand {
    fn default() -> ThreadLimitCommand {
        ThreadLimitCommand {
            command: ThreadLimit,
            arguments: HashMap::new(),
        }
    }
}

impl ThreadLimitCommand {
    pub fn new() -> ThreadLimitCommand {
        let default: ThreadLimitCommand = Default::default();
        ThreadLimitCommand {
            command: default.command,
            arguments: default.arguments,
        }
    }

    pub fn max(mut self, max: u64) -> ThreadLimitCommand {
        let string = format!("{}", max);
        self.arguments.insert("max".to_string(), string.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ThreadLimit, query)
    }
}

impl Queryable for ThreadLimitCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ThreadLimitCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ThreadLimitCommand);
request_cancellable!(ThreadLimitCommand);
request_timeoutable!(ThreadLimitCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::ThreadLimit;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let thread_limit = ThreadLimitCommand::new();
        let expected = ThreadLimitCommand {
            command: ThreadLimit,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, thread_limit);
    }

    #[test]
    fn test_max() {
        let thread_limit = ThreadLimitCommand::new().max(4);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "4".to_string());
        let expected = ThreadLimitCommand {
            command: ThreadLimit,
            arguments: arg.to_owned(),
        };
        assert_eq!(expected, thread_limit);
    }

    #[test]
    fn test_build() {
        let actual = ThreadLimitCommand::new().max(2).build();
        let expected_query: Query = vec![("max".to_string(), "2".to_string())];
        let expected = (ThreadLimit, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = ThreadLimitCommand::new().max(4).to_query();
        let url_encoded = "/d/thread_limit?max=4";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = ThreadLimitCommand::new().max(8).to_command();
        let cli_encoded = "thread_limit --max 8";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ThreadLimitCommand {
            command: ThreadLimit,
            arguments: arg.to_owned(),
        };
        let query = ThreadLimitCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
