use command::{Command, Query};
use command::Command::TableRemove;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TableRemoveCommand {
    command: Command,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for TableRemoveCommand {
    fn default() -> TableRemoveCommand {
        TableRemoveCommand {
            command: TableRemove,
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl TableRemoveCommand {
    pub fn new(name: String) -> TableRemoveCommand {
        TableRemoveCommand {
            name: name,
            ..TableRemoveCommand::default()
        }
    }

    pub fn dependent(mut self, dependent: bool) -> TableRemoveCommand {
        let flag = if dependent { "yes" } else { "no" };
        self.arguments.insert("dependent".to_string(), flag.to_string());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (TableRemove, query)
    }
}

impl Queryable for TableRemoveCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for TableRemoveCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(TableRemoveCommand);
request_cancellable!(TableRemoveCommand);
request_timeoutable!(TableRemoveCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::TableRemove;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let table_remove = TableRemoveCommand::new("Table".to_string());
        let expected = TableRemoveCommand {
            command: TableRemove,
            name: "Table".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, table_remove);
    }

    #[test]
    fn test_dependent() {
        let dependent_yes = TableRemoveCommand::new("Table".to_string()).dependent(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("dependent".to_string(), "yes".to_string());
        let expected_yes = TableRemoveCommand {
            command: TableRemove,
            name: "Table".to_string(),
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, dependent_yes);
        let dependent_no = TableRemoveCommand::new("Table".to_string()).dependent(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("dependent".to_string(), "no".to_string());
        let expected_no = TableRemoveCommand {
            command: TableRemove,
            name: "Table".to_string(),
            arguments: arg_no,
        };
        assert_eq!(expected_no, dependent_no);
    }

    #[test]
    fn test_build() {
        let actual = TableRemoveCommand::new("Table".to_string()).build();
        let expected_query: Query = vec![("name".to_string(), "Table".to_string())];
        let expected = (TableRemove, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = TableRemoveCommand::new("Table".to_string()).to_query();
        let url_encoded = "/d/table_remove?name=Table";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = TableRemoveCommand::new("Table".to_string()).to_command();
        let cli_encoded = "table_remove --name Table";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = TableRemoveCommand {
            command: TableRemove,
            name: "Table".to_string(),
            arguments: arg.to_owned(),
        };
        let query = TableRemoveCommand::new("Table".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
