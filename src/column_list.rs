use super::command::{Command, Query};
use super::command::Command::ColumnList;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColumnListCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for ColumnListCommand {
    fn default() -> ColumnListCommand {
        ColumnListCommand {
            command: ColumnList,
            arguments: HashMap::new(),
        }
    }
}

impl ColumnListCommand {
    pub fn new() -> ColumnListCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ColumnList, query)
    }
}

impl Queryable for ColumnListCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ColumnListCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ColumnListCommand);
request_cancellable!(ColumnListCommand);
request_timeoutable!(ColumnListCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::ColumnList;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let column_list = ColumnListCommand::new();
        let expected = ColumnListCommand {
            command: ColumnList,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, column_list);
    }

    #[test]
    fn test_build() {
        let actual = ColumnListCommand::new().build();
        let expected_query: Query = vec![];
        let expected = (ColumnList, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = ColumnListCommand::new().to_query();
        let url_encoded = "/d/column_list?";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = ColumnListCommand::new().to_command();
        let cli_encoded = "column_list";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ColumnListCommand {
            command: ColumnList,
            arguments: arg.to_owned(),
        };
        let query = ColumnListCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
