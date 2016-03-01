use super::command::{Command, Query};
use super::command::Command::Status;

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

#[cfg(test)]
mod test {
    use super::*;
    use command::Command::Status;

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
}
