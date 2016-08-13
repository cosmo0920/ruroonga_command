use super::command::{Command, Query};
use super::command::Command::LogicalTableRemove;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;
use types::range_filter_type::RangeFilterType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogicalTableRemoveCommand {
    command: Command,
    logical_table: String,
    shard_key: String,
    arguments: HashMap<String, String>,
}

impl Default for LogicalTableRemoveCommand {
    fn default() -> LogicalTableRemoveCommand {
        LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "".to_string(),
            shard_key: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl LogicalTableRemoveCommand {
    pub fn new(logical_table: String, shard_key: String) -> LogicalTableRemoveCommand {
        let default: LogicalTableRemoveCommand = Default::default();
        LogicalTableRemoveCommand {
            command: default.command,
            logical_table: logical_table,
            shard_key: shard_key,
            arguments: default.arguments,
        }
    }

    pub fn min(mut self, min: String) -> LogicalTableRemoveCommand {
        let encoded = format!("'{}'", min);
        self.arguments.insert("min".to_string(), encoded.to_owned());
        self
    }

    pub fn min_border(mut self, border: RangeFilterType) -> LogicalTableRemoveCommand {
        let encoded = format!("'{}'", border);
        self.arguments.insert("min_border".to_string(), encoded.to_owned());
        self
    }

    pub fn max(mut self, max: String) -> LogicalTableRemoveCommand {
        let encoded = format!("'{}'", max);
        self.arguments.insert("max".to_string(), encoded.to_owned());
        self
    }

    pub fn max_border(mut self, border: RangeFilterType) -> LogicalTableRemoveCommand {
        let encoded = format!("'{}'", border);
        self.arguments.insert("max_border".to_string(), encoded.to_owned());
        self
    }

    pub fn dependent(mut self, dependent: bool) -> LogicalTableRemoveCommand {
        let flag = if dependent { "yes" } else { "no" };
        self.arguments.insert("dependent".to_string(), flag.to_string());
        self
    }


    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("logical_table".to_string(), self.logical_table),
                                    ("shard_key".to_string(), self.shard_key)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (LogicalTableRemove, query)
    }
}

impl Queryable for LogicalTableRemoveCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LogicalTableRemoveCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(LogicalTableRemoveCommand);
request_cancellable!(LogicalTableRemoveCommand);
request_timeoutable!(LogicalTableRemoveCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::LogicalTableRemove;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;
    use types::range_filter_type::RangeFilterType::{Include, Exclude};

    #[test]
    fn test_new() {
        let logical_table_remove = LogicalTableRemoveCommand::new("Logs".to_string(),
                                                                  "timestamp".to_string());
        let expected = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, logical_table_remove);
    }

    #[test]
    fn test_min() {
        let select = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string())
            .min("2016/08/10 00:00:00".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("min".to_string(), "'2016/08/10 00:00:00'".to_string());
        let expected = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_min_border() {
        let select = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string())
            .min("2016/08/10 00:00:00".to_string())
            .min_border(Exclude);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("min".to_string(), "'2016/08/10 00:00:00'".to_string());
        arg.insert("min_border".to_string(), "'exclude'".to_string());
        let expected = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_max() {
        let select = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string())
            .max("2016/08/10 00:00:00".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "'2016/08/10 00:00:00'".to_string());
        let expected = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_max_border() {
        let select = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string())
            .max("2016/08/10 00:00:00".to_string())
            .max_border(Include);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "'2016/08/10 00:00:00'".to_string());
        arg.insert("max_border".to_string(), "'include'".to_string());
        let expected = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_dependent() {
        let dependent_yes = LogicalTableRemoveCommand::new("Logs".to_string(),
                                                           "timestamp".to_string())
            .dependent(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("dependent".to_string(), "yes".to_string());
        let expected_yes = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, dependent_yes);
        let dependent_no = LogicalTableRemoveCommand::new("Logs".to_string(),
                                                          "timestamp".to_string())
            .dependent(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("dependent".to_string(), "no".to_string());
        let expected_no = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg_no,
        };
        assert_eq!(expected_no, dependent_no);
    }

    #[test]
    fn test_build() {
        let actual = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string())
            .build();
        let expected_query: Query = vec![("logical_table".to_string(), "Logs".to_string()),
                                         ("shard_key".to_string(), "timestamp".to_string())];
        let expected = (LogicalTableRemove, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string())
            .to_query();
        let url_encoded = "/d/logical_table_remove?logical_table=Logs&shard_key=timestamp";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string())
            .to_command();
        let cli_encoded = "logical_table_remove --logical_table Logs --shard_key timestamp";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = LogicalTableRemoveCommand {
            command: LogicalTableRemove,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg.to_owned(),
        };
        let query = LogicalTableRemoveCommand::new("Logs".to_string(), "timestamp".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
