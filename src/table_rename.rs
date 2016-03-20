use super::command::{Command, Query};
use super::command::Command::TableRename;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TableRenameCommand {
    command: Command,
    name: String,
    new_name: String,
    arguments: HashMap<String, String>,
}

impl Default for TableRenameCommand {
    fn default() -> TableRenameCommand {
        TableRenameCommand {
            command: TableRename,
            name: "".to_string(),
            new_name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl TableRenameCommand {
    pub fn new(name: String, new_name: String) -> TableRenameCommand {
        let default: TableRenameCommand = Default::default();
        TableRenameCommand {
            command: default.command,
            name: name,
            new_name: new_name,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name),
                                    ("new_name".to_string(), self.new_name)];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (TableRename, query)
    }
}

impl Queryable for TableRenameCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for TableRenameCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(TableRenameCommand);
request_cancellable!(TableRenameCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::TableRename;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let table_rename = TableRenameCommand::new("Books".to_string(), "SoldBooks".to_string());
        let expected = TableRenameCommand {
            command: TableRename,
            name: "Books".to_string(),
            new_name: "SoldBooks".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, table_rename);
    }

    #[test]
    fn test_build() {
        let actual = TableRenameCommand::new("Books".to_string(), "SoldBooks".to_string()).build();
        let expected_query: Query = vec![("name".to_string(), "Books".to_string()),
                                         ("new_name".to_string(), "SoldBooks".to_string())];
        let expected = (TableRename, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = TableRenameCommand::new("Books".to_string(), "SoldBooks".to_string())
                        .to_query();
        let url_encoded = "/d/table_rename?name=Books&new_name=SoldBooks";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = TableRenameCommand::new("Books".to_string(), "SoldBooks".to_string())
                        .to_command();
        let cli_encoded = "table_rename --name Books --new_name SoldBooks";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = TableRenameCommand {
            command: TableRename,
            name: "Books".to_string(),
            new_name: "SoldBooks".to_string(),
            arguments: arg.clone(),
        };
        let query = TableRenameCommand::new("Books".to_string(), "SoldBooks".to_string());
        unsafe {
            let extended = query.set_arguments(arg.clone());
            assert_eq!(expected, extended);
        }
    }
}
