use command::{Command, Query};
use command::Command::LogicalCount;
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
pub struct LogicalCountCommand {
    command: Command,
    logical_table: String,
    shard_key: String,
    arguments: HashMap<String, String>,
}

impl Default for LogicalCountCommand {
    fn default() -> LogicalCountCommand {
        LogicalCountCommand {
            command: LogicalCount,
            logical_table: "".to_string(),
            shard_key: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl LogicalCountCommand {
    pub fn new(logical_table: String, shard_key: String) -> LogicalCountCommand {
        let default: LogicalCountCommand = Default::default();
        LogicalCountCommand {
            command: default.command,
            logical_table: logical_table,
            shard_key: shard_key,
            arguments: default.arguments,
        }
    }

    pub fn filter(mut self, filter: String) -> LogicalCountCommand {
        let encoded = format!("'{}'", filter);
        self.arguments.insert("filter".to_string(), encoded.to_owned());
        self
    }

    pub fn min(mut self, min: String) -> LogicalCountCommand {
        let encoded = format!("'{}'", min);
        self.arguments.insert("min".to_string(), encoded.to_owned());
        self
    }

    pub fn min_border(mut self, border: RangeFilterType) -> LogicalCountCommand {
        let encoded = format!("'{}'", border);
        self.arguments.insert("min_border".to_string(), encoded.to_owned());
        self
    }

    pub fn max(mut self, max: String) -> LogicalCountCommand {
        let encoded = format!("'{}'", max);
        self.arguments.insert("max".to_string(), encoded.to_owned());
        self
    }

    pub fn max_border(mut self, border: RangeFilterType) -> LogicalCountCommand {
        let encoded = format!("'{}'", border);
        self.arguments.insert("max_border".to_string(), encoded.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("logical_table".to_string(), self.logical_table),
                                    ("shard_key".to_string(), self.shard_key)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (LogicalCount, query)
    }
}

impl Queryable for LogicalCountCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LogicalCountCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(LogicalCountCommand);
request_cancellable!(LogicalCountCommand);
request_timeoutable!(LogicalCountCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::LogicalCount;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;
    use types::range_filter_type::RangeFilterType::{Exclude, Include};

    #[test]
    fn test_new() {
        let logical_shard_list = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string());
        let expected = LogicalCountCommand {
            command: LogicalCount,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, logical_shard_list);
    }

    #[test]
    fn test_filter() {
        let select = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string())
            .filter("output_column @ \"type_safe\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("filter".to_string(),
                   "'output_column @ \"type_safe\"'".to_string());
        let expected = LogicalCountCommand {
            command: LogicalCount,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_min() {
        let select = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string())
            .min("2016/09/04 00:00:00".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("min".to_string(), "'2016/09/04 00:00:00'".to_string());
        let expected = LogicalCountCommand {
            command: LogicalCount,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_min_border() {
        let select = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string())
            .min("2016/09/04 00:00:00".to_string())
            .min_border(Exclude);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("min".to_string(), "'2016/09/04 00:00:00'".to_string());
        arg.insert("min_border".to_string(), "'exclude'".to_string());
        let expected = LogicalCountCommand {
            command: LogicalCount,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_max() {
        let select = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string())
            .max("2016/09/04 00:00:00".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "'2016/09/04 00:00:00'".to_string());
        let expected = LogicalCountCommand {
            command: LogicalCount,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_max_border() {
        let select = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string())
            .max("2016/09/04 00:00:00".to_string())
            .max_border(Include);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "'2016/09/04 00:00:00'".to_string());
        arg.insert("max_border".to_string(), "'include'".to_string());
        let expected = LogicalCountCommand {
            command: LogicalCount,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_build() {
        let actual = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string()).build();
        let expected_query: Query = vec![("logical_table".to_string(), "Logs".to_string()),
                                         ("shard_key".to_string(),"timestamp".to_string())];
        let expected = (LogicalCount, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string()).to_query();
        let url_encoded = "/d/logical_count?logical_table=Logs&shard_key=timestamp";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string()).to_command();
        let cli_encoded = "logical_count --logical_table Logs --shard_key timestamp";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = LogicalCountCommand {
            command: LogicalCount,
            logical_table: "Logs".to_string(),
            shard_key: "timestamp".to_string(),
            arguments: arg.to_owned(),
        };
        let query = LogicalCountCommand::new("Logs".to_string(), "timestamp".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
