use command::{Command, Query};
use command::Command::TableList;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TableListCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for TableListCommand {
    fn default() -> TableListCommand {
        TableListCommand {
            command: TableList,
            arguments: HashMap::new(),
        }
    }
}

impl TableListCommand {
    pub fn new() -> TableListCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (TableList, query)
    }
}

impl Queryable for TableListCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for TableListCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(TableListCommand);
request_cancellable!(TableListCommand);
request_timeoutable!(TableListCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::TableList;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let table_list = TableListCommand::new();
        let expected = TableListCommand {
            command: TableList,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, table_list);
    }

    #[test]
    fn test_build() {
        let actual = TableListCommand::new().build();
        let expected_query: Query = vec![];
        let expected = (TableList, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = TableListCommand::new().to_query();
        let url_encoded = "/d/table_list?";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = TableListCommand::new().to_command();
        let cli_encoded = "table_list";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = TableListCommand {
            command: TableList,
            arguments: arg.to_owned(),
        };
        let query = TableListCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
