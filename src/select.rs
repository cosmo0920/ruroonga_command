use super::command::{Query, Command};
use super::command::Command::Select;
use std::iter::IntoIterator;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectCommand {
    command: Command,
    arguments: Query,
    table: String,
}

impl Default for SelectCommand {
    fn default() -> SelectCommand {
        SelectCommand {
            command: Select,
            arguments: vec![],
            table: "".to_string(),
        }
    }
}

impl SelectCommand {
    pub fn new(table: String) -> SelectCommand {
        let default: SelectCommand = Default::default();
        SelectCommand {
            command: default.command,
            arguments: default.arguments,
            table: table
        }
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> SelectCommand {
        fn split_columns_vec(columns: Vec<String>) -> String {
            let string = columns.into_iter().collect::<Vec<String>>()
                .join(",").to_owned();
            string.clone()
        }

        let string = split_columns_vec(columns);
        let argument = ("output_columns".to_string(), string.clone());
        self.arguments.push(argument.clone());
        self
    }

    pub fn offset(mut self, offset: u64) -> SelectCommand {
        let offset = format!("{}", offset);
        let argument = ("offset".to_string(), offset.clone());
        self.arguments.push(argument.clone());
        self
    }

    pub fn limit(mut self, limit: u64) -> SelectCommand {
        let limit = format!("{}", limit);
        let argument = ("limit".to_string(), limit.clone());
        self.arguments.push(argument.clone());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use command::Command::Select;

    #[test]
    fn test_new() {
        let vanilla_select = SelectCommand::new("test".to_string());
        let expected = SelectCommand {
            command: Select,
            arguments: vec![],
            table: "test".to_string(),
        };
        assert_eq!(expected, vanilla_select);
    }

    #[test]
    fn test_output_columns() {
        let select = SelectCommand::new("test".to_string())
            .output_columns(vec!["test".to_string(), "piyo".to_string()]);
        let expected = SelectCommand {
            command: Select,
            arguments: vec![("output_columns".to_string(),
                             "test,piyo".to_string())],
            table: "test".to_string(),
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_offset() {
        let select = SelectCommand::new("test".to_string())
            .offset(100);
        let expected = SelectCommand {
            command: Select,
            arguments: vec![("offset".to_string(),
                             "100".to_string())],
            table: "test".to_string(),
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_limit() {
        let select = SelectCommand::new("test".to_string())
            .limit(50);
        let expected = SelectCommand {
            command: Select,
            arguments: vec![("limit".to_string(),
                             "50".to_string())],
            table: "test".to_string(),
        };
        assert_eq!(expected, select);
    }
}
