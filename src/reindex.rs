use super::command::{Command, Query};
use super::command::Command::Reindex;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReindexCommand {
    command: Command,
    target_name: String,
    arguments: HashMap<String, String>,
}

impl Default for ReindexCommand {
    fn default() -> ReindexCommand {
        ReindexCommand {
            command: Reindex,
            target_name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl ReindexCommand {
    pub fn new(target_name: String) -> ReindexCommand {
        let default: ReindexCommand = Default::default();
        ReindexCommand {
            command: default.command,
            target_name: target_name,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("target_name".to_string(), self.target_name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Reindex, query)
    }
}

impl Queryable for ReindexCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ReindexCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ReindexCommand);
request_cancellable!(ReindexCommand);
request_timeoutable!(ReindexCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Reindex;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let truncate = ReindexCommand::new("Timestamp".to_string());
        let expected = ReindexCommand {
            command: Reindex,
            target_name: "Timestamp".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, truncate);
    }

    #[test]
    fn test_build() {
        let actual = ReindexCommand::new("Timestamp".to_string()).build();
        let expected_query: Query = vec![("target_name".to_string(), "Timestamp".to_string())];
        let expected = (Reindex, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = ReindexCommand::new("Timestamp".to_string()).to_query();
        let url_encoded = "/d/reindex?target_name=Timestamp";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = ReindexCommand::new("Timestamp.broken".to_string()).to_command();
        let cli_encoded = "reindex --target_name Timestamp.broken";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ReindexCommand {
            command: Reindex,
            target_name: "Timestamp".to_string(),
            arguments: arg.to_owned(),
        };
        let query = ReindexCommand::new("Timestamp".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
