use super::command::{Command, Query};
use super::command::Command::ColumnRemove;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColumnRemoveCommand {
    command: Command,
    table: String,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for ColumnRemoveCommand {
    fn default() -> ColumnRemoveCommand {
        ColumnRemoveCommand {
            command: ColumnRemove,
            table: "".to_string(),
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl ColumnRemoveCommand {
    pub fn new(table: String, name: String) -> ColumnRemoveCommand {
        let default: ColumnRemoveCommand = Default::default();
        ColumnRemoveCommand {
            command: default.command,
            table: table,
            name: name,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("table".to_string(), self.table),
                                    ("name".to_string(), self.name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ColumnRemove, query)
    }
}

impl Queryable for ColumnRemoveCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ColumnRemoveCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ColumnRemoveCommand);
request_cancellable!(ColumnRemoveCommand);
request_timeoutable!(ColumnRemoveCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::ColumnRemove;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let table_rename = ColumnRemoveCommand::new("Books".to_string(), "category".to_string());
        let expected = ColumnRemoveCommand {
            command: ColumnRemove,
            table: "Books".to_string(),
            name: "category".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, table_rename);
    }

    #[test]
    fn test_build() {
        let actual = ColumnRemoveCommand::new("Books".to_string(), "category".to_string()).build();
        let expected_query: Query = vec![("table".to_string(), "Books".to_string()),
                                         ("name".to_string(), "category".to_string())];
        let expected = (ColumnRemove, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = ColumnRemoveCommand::new("Books".to_string(), "category".to_string())
                        .to_query();
        let url_encoded = "/d/column_remove?table=Books&name=category";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = ColumnRemoveCommand::new("Books".to_string(), "category".to_string())
                        .to_command();
        let cli_encoded = "column_remove --table Books --name category";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ColumnRemoveCommand {
            command: ColumnRemove,
            table: "Books".to_string(),
            name: "category".to_string(),
            arguments: arg.to_owned(),
        };
        let query = ColumnRemoveCommand::new("Books".to_string(), "category".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
