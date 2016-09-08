use command::{Command, Query};
use command::Command::LogicalShardList;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogicalShardListCommand {
    command: Command,
    logical_table: String,
    arguments: HashMap<String, String>,
}

impl Default for LogicalShardListCommand {
    fn default() -> LogicalShardListCommand {
        LogicalShardListCommand {
            command: LogicalShardList,
            logical_table: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl LogicalShardListCommand {
    pub fn new(logical_table: String) -> LogicalShardListCommand {
        LogicalShardListCommand {
            logical_table: logical_table,
            ..LogicalShardListCommand::default()
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("logical_table".to_string(), self.logical_table)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (LogicalShardList, query)
    }
}

impl Queryable for LogicalShardListCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LogicalShardListCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(LogicalShardListCommand);
request_cancellable!(LogicalShardListCommand);
request_timeoutable!(LogicalShardListCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::LogicalShardList;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let logical_shard_list = LogicalShardListCommand::new("Logs".to_string());
        let expected = LogicalShardListCommand {
            command: LogicalShardList,
            logical_table: "Logs".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, logical_shard_list);
    }

    #[test]
    fn test_build() {
        let actual = LogicalShardListCommand::new("Logs".to_string()).build();
        let expected_query: Query = vec![("logical_table".to_string(), "Logs".to_string())];
        let expected = (LogicalShardList, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = LogicalShardListCommand::new("Logs".to_string()).to_query();
        let url_encoded = "/d/logical_shard_list?logical_table=Logs";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = LogicalShardListCommand::new("Logs".to_string()).to_command();
        let cli_encoded = "logical_shard_list --logical_table Logs";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = LogicalShardListCommand {
            command: LogicalShardList,
            logical_table: "Logs".to_string(),
            arguments: arg.to_owned(),
        };
        let query = LogicalShardListCommand::new("Logs".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
