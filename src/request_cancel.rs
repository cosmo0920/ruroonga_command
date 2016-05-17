use super::command::{Command, Query};
use super::command::Command::RequestCancel;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RequestCancelCommand {
    command: Command,
    id: String,
    arguments: HashMap<String, String>,
}

impl Default for RequestCancelCommand {
    fn default() -> RequestCancelCommand {
        RequestCancelCommand {
            command: RequestCancel,
            id: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl RequestCancelCommand {
    pub fn new(id: String) -> RequestCancelCommand {
        let default: RequestCancelCommand = Default::default();
        RequestCancelCommand {
            command: default.command,
            id: id,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("id".to_string(), self.id)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (self.command, query)
    }
}

impl Queryable for RequestCancelCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for RequestCancelCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

request_cancellable!(RequestCancelCommand);
request_timeoutable!(RequestCancelCommand);

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use command::Command::RequestCancel;
    use queryable::Queryable;
    use commandable::Commandable;

    #[test]
    fn test_new() {
        let status = RequestCancelCommand::new("cancel-id".to_string());
        let expected = RequestCancelCommand {
            command: RequestCancel,
            id: "cancel-id".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, status);
    }

    #[test]
    fn test_build() {
        let actual = RequestCancelCommand::new("cancel-id".to_string()).build();
        let expected = (RequestCancel, vec![("id".to_string(), "cancel-id".to_string())]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = RequestCancelCommand::new("cancel-id".to_string()).to_query();
        let url_encoded = "/d/request_cancel?id=cancel-id";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = RequestCancelCommand::new("cancel-id".to_string()).to_command();
        let cli_encoded = "request_cancel --id cancel-id";
        assert_eq!(cli_encoded.to_string(), query);
    }
}
