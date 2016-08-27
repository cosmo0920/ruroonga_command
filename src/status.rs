use command::{Command, Query};
use command::Command::Status;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatusCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for StatusCommand {
    fn default() -> StatusCommand {
        StatusCommand {
            command: Status,
            arguments: HashMap::new(),
        }
    }
}

impl StatusCommand {
    pub fn new() -> StatusCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (self.command, query)
    }
}

impl Queryable for StatusCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for StatusCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

request_cancellable!(StatusCommand);
request_timeoutable!(StatusCommand);

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use command::Command::Status;
    use queryable::Queryable;
    use commandable::Commandable;

    #[test]
    fn test_new() {
        let status = StatusCommand::new();
        let expected = StatusCommand {
            command: Status,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, status);
    }

    #[test]
    fn test_build() {
        let actual = StatusCommand::new().build();
        let expected = (Status, vec![]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = StatusCommand::new().to_query();
        let url_encoded = "/d/status?";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = StatusCommand::new().to_command();
        let cli_encoded = "status";
        assert_eq!(cli_encoded.to_string(), query);
    }
}
