use super::command::{Command, Query};
use super::command::Command::Delete;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;

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
        self.arguments.insert("id".to_string(), id_str.clone());
        self
    }

    pub fn filter(mut self, filter: String) -> DeleteCommand {
        let encoded = format!("'{}'", filter);
        self.arguments.insert("filter".to_string(), encoded.clone());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("table".to_string(), self.table)];
        for (key, value) in self.arguments.iter() {
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

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Delete;
    use std::collections::HashMap;
    use queryable::Queryable;

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
        let delete = DeleteCommand::new("Books".to_string())
            .key("tutorial".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("key".to_string(),
                   "tutorial".to_string());
        let expected = DeleteCommand {
            command: Delete,
            table: "Books".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, delete);
    }

    #[test]
    fn test_id() {
        let delete = DeleteCommand::new("Books".to_string())
            .id(30);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("id".to_string(),
                   "30".to_string());
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
        arg.insert("filter".to_string(),
                   "'author == unknown'".to_string());
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
            .filter("author == unknown".to_string()).build();
        let expected_query: Query =
            vec![("table".to_string(), "Books".to_string()),
                 ("filter".to_string(),
                  "'author == unknown'".to_string())];
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
}
