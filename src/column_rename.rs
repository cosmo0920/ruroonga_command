use command::{Command, Query};
use command::Command::ColumnRename;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColumnRenameCommand {
    command: Command,
    name: String,
    new_name: String,
    arguments: HashMap<String, String>,
}

impl Default for ColumnRenameCommand {
    fn default() -> ColumnRenameCommand {
        ColumnRenameCommand {
            command: ColumnRename,
            name: "".to_string(),
            new_name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl ColumnRenameCommand {
    pub fn new(name: String, new_name: String) -> ColumnRenameCommand {
        ColumnRenameCommand {
            name: name,
            new_name: new_name,
            ..ColumnRenameCommand::default()
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name),
                                    ("new_name".to_string(), self.new_name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ColumnRename, query)
    }
}

impl Queryable for ColumnRenameCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ColumnRenameCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ColumnRenameCommand);
request_cancellable!(ColumnRenameCommand);
request_timeoutable!(ColumnRenameCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::ColumnRename;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let table_rename = ColumnRenameCommand::new("old".to_string(), "new".to_string());
        let expected = ColumnRenameCommand {
            command: ColumnRename,
            name: "old".to_string(),
            new_name: "new".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, table_rename);
    }

    #[test]
    fn test_build() {
        let actual = ColumnRenameCommand::new("old".to_string(), "new".to_string()).build();
        let expected_query: Query = vec![("name".to_string(), "old".to_string()),
                                         ("new_name".to_string(), "new".to_string())];
        let expected = (ColumnRename, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = ColumnRenameCommand::new("old".to_string(), "new".to_string()).to_query();
        let url_encoded = "/d/column_rename?name=old&new_name=new";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = ColumnRenameCommand::new("old".to_string(), "new".to_string()).to_command();
        let cli_encoded = "column_rename --name old --new_name new";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ColumnRenameCommand {
            command: ColumnRename,
            name: "old".to_string(),
            new_name: "new".to_string(),
            arguments: arg.to_owned(),
        };
        let query = ColumnRenameCommand::new("old".to_string(), "new".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
