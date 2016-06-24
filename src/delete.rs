use super::command::{Command, Query};
use super::command::Command::Delete;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeleteCommand {
    command: Command,
    table: String,
    arguments: HashMap<String, String>,
}

impl Default for DeleteCommand {
    fn default() -> DeleteCommand {
        DeleteCommand {
            command: Delete,
            table: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl DeleteCommand {
    pub fn new(table: String) -> DeleteCommand {
        let default: DeleteCommand = Default::default();
        DeleteCommand {
            command: default.command,
            table: table,
            arguments: default.arguments,
        }
    }

    pub fn key(mut self, key: String) -> DeleteCommand {
        self.arguments.insert("key".to_string(), key);
        self
    }

    pub fn id(mut self, id: u64) -> DeleteCommand {
        let id_str = format!("{}", id);
        self.arguments.insert("id".to_string(), id_str.to_owned());
        self
    }

    pub fn filter(mut self, filter: String) -> DeleteCommand {
        let encoded = format!("'{}'", filter);
        self.arguments.insert("filter".to_string(), encoded.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("table".to_string(), self.table)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Delete, query)
    }
}

impl Queryable for DeleteCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for DeleteCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(DeleteCommand);
request_cancellable!(DeleteCommand);
request_timeoutable!(DeleteCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Delete;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let vanilla_delete = DeleteCommand::new("Books".to_string());
        let expected = DeleteCommand {
            command: Delete,
            table: "Books".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_delete);
    }

    #[test]
    fn test_key() {
        let delete = DeleteCommand::new("Books".to_string()).key("tutorial".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("key".to_string(), "tutorial".to_string());
        let expected = DeleteCommand {
            command: Delete,
            table: "Books".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, delete);
    }

    #[test]
    fn test_id() {
        let delete = DeleteCommand::new("Books".to_string()).id(30);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("id".to_string(), "30".to_string());
        let expected = DeleteCommand {
            command: Delete,
            table: "Books".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, delete);
    }

    #[test]
    fn test_filter() {
        let delete = DeleteCommand::new("Books".to_string())
            .filter("author == unknown".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("filter".to_string(), "'author == unknown'".to_string());
        let expected = DeleteCommand {
            command: Delete,
            table: "Books".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, delete);
    }

    #[test]
    fn test_build() {
        let actual = DeleteCommand::new("Books".to_string())
            .filter("author == unknown".to_string())
            .build();
        let expected_query: Query = vec![("table".to_string(), "Books".to_string()),
                                         ("filter".to_string(), "'author == unknown'".to_string())];
        let expected = (Delete, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = DeleteCommand::new("Books".to_string())
            .filter("author == unknown".to_string())
            .to_query();
        let url_encoded = "/d/delete?table=Books&filter=%27author+%3D%3D+unknown%27";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = DeleteCommand::new("Books".to_string())
            .filter("author == unknown".to_string())
            .to_command();
        let cli_encoded = "delete --table Books --filter \'author == unknown\'";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = DeleteCommand {
            command: Delete,
            table: "Test".to_string(),
            arguments: arg.to_owned(),
        };
        let query = DeleteCommand::new("Test".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
