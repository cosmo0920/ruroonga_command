use super::command::{Command, Query};
use super::command::Command::Load;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::InputType::{Json, ExtInputType};
use util;

pub type LoadValues = String;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoadCommand {
    command: Command,
    arguments: HashMap<String, String>,
    table: String,
    values: LoadValues,
}

#[derive (Debug)]
pub enum InputTypeError { Empty }

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum InputType {
    Json,
    /// For future extensibility.
    ExtInputType(String),
}

impl AsRef<str> for InputType {
    fn as_ref(&self) -> &str {
        match *self {
            Json => "json",
            ExtInputType(ref s) => s.as_ref()
        }
    }
}

impl fmt::Display for InputType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Json => "json",
            ExtInputType(ref s) => s.as_ref()
        })
    }
}

impl FromStr for InputType {
    type Err = InputTypeError;
    fn from_str(s: &str) -> Result<InputType, InputTypeError> {
        if s == "" {
            Err(InputTypeError::Empty)
        } else {
            Ok(match s {
                "json" => Json,
                _ => ExtInputType(s.to_owned())
            })
        }
    }
}

impl Default for LoadCommand {
    fn default() -> LoadCommand {
        LoadCommand {
            command: Load,
            table: "".to_string(),
            arguments: HashMap::new(),
            values: "".to_string(),
        }
    }
}

impl LoadCommand {
    pub fn new(table: String, data: String) -> LoadCommand {
        let default: LoadCommand = Default::default();
        LoadCommand {
            command: default.command,
            table: table,
            arguments: default.arguments,
            values: data,
        }
    }

    pub fn columns(mut self, columns: Vec<String>) -> LoadCommand {
        let string = util::split_values_vec(columns);
        self.arguments.insert("columns".to_string(), string.clone());
        self
    }

    pub fn input_type(mut self, input_type: InputType) -> LoadCommand {
        let input_type_str = format!("{}", input_type);
        self.arguments.insert("input_type".to_string(),
                              input_type_str.clone());
        self
    }

    pub fn build(self) -> (Command, Query, LoadValues) {
        let mut query: Query = vec![("table".to_string(), self.table)];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        let values = self.values.clone();
        (Load, query, values)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Load;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(InputType::Json, FromStr::from_str("json").unwrap());
        assert_eq!(InputType::ExtInputType("added-type".to_owned()),
                   FromStr::from_str("added-type").unwrap());
        let x: Result<InputType, _> = FromStr::from_str("");
        if let Err(InputTypeError::Empty) = x {
        } else {
            panic!("An empty input type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("json".to_owned(), format!("{}", InputType::Json));
        assert_eq!("added-type".to_owned(),
                   format!("{}", InputType::ExtInputType("added-type".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(InputType::Json.as_ref(), "json");
        assert_eq!(InputType::ExtInputType("added-type".to_owned()).as_ref(),
                   "added-type");
    }

    const DATA: &'static str = r#"[
{"_key":"http://example.org/","title":"This is test record 1!"},
]"#;

    #[test]
    fn test_new() {
        let vanilla_load = LoadCommand::new("test".to_string(),
                                            DATA.to_string());
        let expected = LoadCommand {
            command: Load,
            table: "test".to_string(),
            arguments: HashMap::new(),
            values: DATA.to_string(),
        };
        assert_eq!(expected, vanilla_load);
    }

    #[test]
    fn test_columns() {
        let load = LoadCommand::new("test".to_string(), DATA.to_string())
            .columns(vec!["_key".to_string(), "title".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("columns".to_string(), "_key,title".to_string());
        let expected = LoadCommand {
            command: Load,
            table: "test".to_string(),
            arguments: arg,
            values: DATA.to_string(),
        };
        assert_eq!(expected, load);
    }

    #[test]
    fn test_input_type() {
        let load = LoadCommand::new("test".to_string(), DATA.to_string())
            .input_type(InputType::Json);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("input_type".to_string(), "json".to_string());
        let expected = LoadCommand {
            command: Load,
            table: "test".to_string(),
            arguments: arg,
            values: DATA.to_string(),
        };
        assert_eq!(expected, load);
    }

    #[test]
    fn test_build() {
        let load = LoadCommand::new("test".to_string(), DATA.to_string())
            .columns(vec!["_key".to_string(), "title".to_string()]).build();
        let expected_query: Query =
            vec![("table".to_string(), "test".to_string()),
                 ("columns".to_string(), "_key,title".to_string())];
        let expected = (Load, expected_query, DATA.to_string());
        assert_eq!(expected, load);
    }
}
