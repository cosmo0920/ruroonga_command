use super::command::{Command, Query};
use super::command::Command::Select;
use std::collections::HashMap;
use util;
use command_query::CommandQuery;
use queryable::Queryable;
use commandable::Commandable;
use command_line::CommandLine;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectCommand {
    command: Command,
    table: String,
    arguments: HashMap<String, String>,
}

impl Default for SelectCommand {
    fn default() -> SelectCommand {
        SelectCommand {
            command: Select,
            table: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl SelectCommand {
    pub fn new(table: String) -> SelectCommand {
        let default: SelectCommand = Default::default();
        SelectCommand {
            command: default.command,
            table: table,
            arguments: default.arguments,
        }
    }

    pub fn filter(mut self, filter: String) -> SelectCommand {
        let encoded = format!("'{}'", filter);
        self.arguments.insert("filter".to_string(), encoded.clone());
        self
    }

    pub fn query(mut self, query: String) -> SelectCommand {
        let encoded = format!("'{}'", query);
        self.arguments.insert("query".to_string(), encoded.clone());
        self
    }

    pub fn match_columns(mut self, columns: Vec<String>) -> SelectCommand {
        let string = util::split_values_vec(columns);
        self.arguments.insert("match_columns".to_string(), string.clone());
        self
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> SelectCommand {
        let string = util::split_values_vec(columns);
        self.arguments.insert("output_columns".to_string(), string.clone());
        self
    }

    pub fn offset(mut self, offset: u64) -> SelectCommand {
        let offset = format!("{}", offset);
        self.arguments.insert("offset".to_string(), offset.clone());
        self
    }

    pub fn limit(mut self, limit: u64) -> SelectCommand {
        let limit = format!("{}", limit);
        self.arguments.insert("limit".to_string(), limit.clone());
        self
    }

    pub fn cache(mut self, cache: bool) -> SelectCommand {
        let flag = match cache {
            true => "yes",
            false => "no",
        };
        self.arguments.insert("cache".to_string(), flag.to_string());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("table".to_string(), self.table)];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Select, query)
    }
}

impl Queryable for SelectCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for SelectCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

impl Fragmentable for SelectCommand {
    fn to_fragment(self) -> (OrderedFragment, QueryFragment) {
        let mut select_fragment = HashMap::new();
        let ordered_fragment = vec![("table".to_string(), self.table)];
        for (key, value) in self.arguments.clone() {
            select_fragment.insert(key, value);
        }
        (ordered_fragment, select_fragment)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Select;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;

    #[test]
    fn test_new() {
        let vanilla_select = SelectCommand::new("test".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_select);
    }

    #[test]
    fn test_filter() {
        let select = SelectCommand::new("test".to_string())
                         .filter("output_column @ \"type_safe\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("filter".to_string(),
                   "'output_column @ \"type_safe\"'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_query() {
        let select = SelectCommand::new("test".to_string())
                         .query("_key:\"http://example.org/\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("query".to_string(),
                   "\'_key:\"http://example.org/\"\'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_match_columns() {
        let select = SelectCommand::new("test".to_string())
                         .match_columns(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("match_columns".to_string(), "test,piyo".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_output_columns() {
        let select = SelectCommand::new("test".to_string())
                         .output_columns(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("output_columns".to_string(), "test,piyo".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_offset() {
        let select = SelectCommand::new("test".to_string()).offset(100);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("offset".to_string(), "100".to_string());
        let expected = SelectCommand {
            command: Select,
            arguments: arg,
            table: "test".to_string(),
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_limit() {
        let select = SelectCommand::new("test".to_string()).limit(50);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("limit".to_string(), "50".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_cache() {
        let select_yes = SelectCommand::new("test".to_string()).cache(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("cache".to_string(), "yes".to_string());
        let expected_yes = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, select_yes);
        let select_no = SelectCommand::new("test".to_string()).cache(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("cache".to_string(), "no".to_string());
        let expected_no = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg_no,
        };
        assert_eq!(expected_no, select_no);
    }

    #[test]
    fn test_build() {
        let actual = SelectCommand::new("test".to_string())
                         .filter("output_column @ \"type_safe\"".to_string())
                         .build();
        let expected_query: Query = vec![("table".to_string(), "test".to_string()),
                                         ("filter".to_string(),
                                          "'output_column @ \"type_safe\"'".to_string())];
        let expected = (Select, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = SelectCommand::new("Test".to_string())
                        .filter("output_column @ \"type_safe\"".to_string())
                        .to_query();
        let url_encoded = "/d/select?table=Test&filter=%27output_column+%40+%22type_safe%22%27";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = SelectCommand::new("Test".to_string())
                        .filter("output_column @ \"type_safe\"".to_string())
                        .to_command();
        let url_encoded = "select --table Test --filter \'output_column @ \"type_safe\"\'";
        assert_eq!(url_encoded.to_string(), query);
    }
}
