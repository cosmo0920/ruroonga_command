use command::{Command, Query};
use command::Command::ColumnCreate;
use std::collections::HashMap;
use types::data_type::DataType;
use types::column_flag_type::ColumnFlagType;
use util;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColumnCreateCommand {
    command: Command,
    table: String,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for ColumnCreateCommand {
    fn default() -> ColumnCreateCommand {
        ColumnCreateCommand {
            command: ColumnCreate,
            table: "".to_string(),
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl ColumnCreateCommand {
    pub fn new(table: String, name: String) -> ColumnCreateCommand {
        let default: ColumnCreateCommand = Default::default();
        ColumnCreateCommand {
            command: default.command,
            table: table,
            name: name,
            arguments: default.arguments,
        }
    }

    pub fn column_type(mut self, column_type: DataType) -> ColumnCreateCommand {
        let string = format!("{}", column_type);
        self.arguments.insert("type".to_string(), string.to_owned());
        self
    }

    pub fn flags(mut self, flags: Vec<ColumnFlagType>) -> ColumnCreateCommand {
        let string = util::split_flags_vec(flags);
        self.arguments.insert("flags".to_string(), string.to_owned());
        self
    }

    pub fn source(mut self, source: Vec<String>) -> ColumnCreateCommand {
        let string = util::split_values_vec(source);
        self.arguments.insert("source".to_string(), string.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("table".to_string(), self.table),
                                    ("name".to_string(), self.name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ColumnCreate, query)
    }
}

impl Queryable for ColumnCreateCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ColumnCreateCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ColumnCreateCommand);
request_cancellable!(ColumnCreateCommand);
request_timeoutable!(ColumnCreateCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Command::ColumnCreate;
    use std::collections::HashMap;
    use types::data_type::DataType;
    use types::column_flag_type::ColumnFlagType;
    use command::Query;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let vanilla_column_create = ColumnCreateCommand::new("Test".to_string(),
                                                             "element".to_string());
        let expected = ColumnCreateCommand {
            command: ColumnCreate,
            table: "Test".to_string(),
            name: "element".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_column_create);
    }

    #[test]
    fn test_column_type() {
        let column_create = ColumnCreateCommand::new("Test".to_string(), "element".to_string())
            .column_type(DataType::ShortText);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("type".to_string(), "ShortText".to_string());
        let expected = ColumnCreateCommand {
            command: ColumnCreate,
            table: "Test".to_string(),
            name: "element".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, column_create);
    }

    #[test]
    fn test_flags() {
        let column_create = ColumnCreateCommand::new("Test".to_string(), "element".to_string())
            .flags(vec![(ColumnFlagType::Scalar)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("flags".to_string(), "COLUMN_SCALAR".to_string());
        let expected = ColumnCreateCommand {
            command: ColumnCreate,
            table: "Test".to_string(),
            name: "element".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, column_create);
    }

    #[test]
    fn test_source() {
        let column_create = ColumnCreateCommand::new("Test".to_string(), "element".to_string())
            .source(vec![("Entry".to_string()), ("person".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("source".to_string(), "Entry,person".to_string());
        let expected = ColumnCreateCommand {
            command: ColumnCreate,
            table: "Test".to_string(),
            name: "element".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, column_create);
    }

    #[test]
    fn test_build() {
        let actual = ColumnCreateCommand::new("Test".to_string(), "element".to_string())
            .flags(vec![(ColumnFlagType::Scalar)])
            .build();
        let expected_query: Query = vec![("table".to_string(), "Test".to_string()),
                                         ("name".to_string(), "element".to_string()),
                                         ("flags".to_string(), "COLUMN_SCALAR".to_string())];
        let expected = (ColumnCreate, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let actual = ColumnCreateCommand::new("Test".to_string(), "element".to_string())
            .column_type(DataType::LongText)
            .to_query();
        let expected = "/d/column_create?table=Test&name=element&type=LongText";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_commandable() {
        let actual = ColumnCreateCommand::new("Test".to_string(), "element".to_string())
            .column_type(DataType::LongText)
            .to_command();
        let expected = "column_create --table Test --name element --type LongText";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ColumnCreateCommand {
            command: ColumnCreate,
            table: "Test".to_string(),
            name: "element".to_string(),
            arguments: arg.to_owned(),
        };
        let query = ColumnCreateCommand::new("Test".to_string(), "element".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
