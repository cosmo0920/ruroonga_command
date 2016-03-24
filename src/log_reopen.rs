use super::command::{Command, Query};
use super::command::Command::LogReopen;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use request_cancellable::RequestCancellable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogReopenCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for LogReopenCommand {
    fn default() -> LogReopenCommand {
        LogReopenCommand {
            command: LogReopen,
            arguments: HashMap::new(),
        }
    }
}

impl LogReopenCommand {
    pub fn new() -> LogReopenCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (self.command, query)
    }
}

impl Queryable for LogReopenCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LogReopenCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

request_cancellable!(LogReopenCommand);

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use command::Command::LogReopen;
    use queryable::Queryable;
    use commandable::Commandable;

    #[test]
    fn test_new() {
        let status = LogReopenCommand::new();
        let expected = LogReopenCommand {
            command: LogReopen,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, status);
    }

    #[test]
    fn test_build() {
        let actual = LogReopenCommand::new().build();
        let expected = (LogReopen, vec![]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = LogReopenCommand::new().to_query();
        let url_encoded = "/d/log_reopen?";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = LogReopenCommand::new().to_command();
        let cli_encoded = "log_reopen";
        assert_eq!(cli_encoded.to_string(), query);
    }
}
