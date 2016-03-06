use super::command::{Command, Query};
use super::command::Command::Status;
use command_query::CommandQuery;
use Queryable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatusCommand {
    command: Command,
}

impl Default for StatusCommand {
    fn default() -> StatusCommand {
        StatusCommand {
            command: Status,
        }
    }
}

impl StatusCommand {
    pub fn new() -> StatusCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let query = vec![];
        (Status, query)
    }
}

impl Queryable for StatusCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use command::Command::Status;
    use Queryable;

    #[test]
    fn test_new() {
        let status = StatusCommand::new();
        let expected = StatusCommand {
            command: Status,
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
}
