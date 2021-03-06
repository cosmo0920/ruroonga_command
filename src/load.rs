use command::{Command, Query};
use command::Command::Load;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::InputType::{Json, ExtInputType};
use util;
use queryable::LoadValues;
use command_query::CommandQuery;
use command_line::CommandLine;
use queryable::Queryable;
use queryable::PostQueryable;
use commandable::Commandable;
use commandable::PostCommandable;
use commandable::DataValues;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoadCommand {
    command: Command,
    arguments: HashMap<String, String>,
    table: String,
    values: LoadValues,
}

#[derive (Debug)]
pub enum InputTypeError {
    Empty,
}

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
            ExtInputType(ref s) => s.as_ref(),
        }
    }
}

impl fmt::Display for InputType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Json => "json",
            ExtInputType(ref s) => s.as_ref(),
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
                _ => ExtInputType(s.to_owned()),
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
        LoadCommand {
            table: table,
            values: data,
            ..LoadCommand::default()
        }
    }

    pub fn columns(mut self, columns: Vec<String>) -> LoadCommand {
        let string = util::split_values_vec(columns);
        self.arguments.insert("columns".to_string(), string.to_owned());
        self
    }

    pub fn input_type(mut self, input_type: InputType) -> LoadCommand {
        let input_type_str = format!("{}", input_type);
        self.arguments.insert("input_type".to_string(), input_type_str.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query, LoadValues) {
        let mut query: Query = vec![("table".to_string(), self.table)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        let values = self.values.to_owned();
        (Load, query, values)
    }
}

impl Queryable for LoadCommand {
    fn to_query(self) -> String {
        let (command, query, values) = self.build();
        let mut command = CommandQuery::new(command, query);
        format!("{}&values={}", command.encode(), values)
    }
}

impl PostQueryable for LoadCommand {
    fn to_post_query(self) -> (String, LoadValues) {
        let (command, query, values) = self.build();
        let mut command = CommandQuery::new(command, query);
        (command.encode(), values)
    }
}

impl Commandable for LoadCommand {
    fn to_command(self) -> String {
        let (command, query, values) = self.build();
        let mut command = CommandLine::new(command, query);
        format!("{}\n{}", command.encode(), values)
    }
}

impl PostCommandable for LoadCommand {
    fn to_post_command(self) -> (String, DataValues) {
        let (command, query, values) = self.build();
        let mut command = CommandLine::new(command, query);
        (command.encode(), values)
    }
}

request_cancellable!(LoadCommand);
request_timeoutable!(LoadCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Load;
    use std::collections::HashMap;
    use std::str::FromStr;
    use queryable::Queryable;
    use queryable::PostQueryable;
    use commandable::Commandable;
    use commandable::PostCommandable;

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
        let vanilla_load = LoadCommand::new("test".to_string(), DATA.to_string());
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
            .columns(vec!["_key".to_string(), "title".to_string()])
            .build();
        let expected_query: Query = vec![("table".to_string(), "test".to_string()),
                                         ("columns".to_string(), "_key,title".to_string())];
        let expected = (Load, expected_query, DATA.to_string());
        assert_eq!(expected, load);
    }

    #[test]
    fn test_queryable() {
        let load_data: &'static str = r#"[
{"_key":"http://example.org/","title":"This is test record 1!"},
]"#;
        let query = LoadCommand::new("test".to_string(), load_data.to_string())
            .input_type(InputType::Json)
            .to_query();
        let url_encoded = "/d/load?table=test&input_type=json&values=[\n{\"_key\":\"http:\
                           //example.org/\",\"title\":\"This is test record 1!\"},\n]";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_post_queryable() {
        let load_data: &'static str = r#"[
{"_key":"http://example.org/","title":"This is test record 1!"},
]"#;
        let (query, values) = LoadCommand::new("test".to_string(), load_data.to_string())
            .input_type(InputType::Json)
            .to_post_query();
        let url_encoded = "/d/load?table=test&input_type=json";
        assert_eq!(url_encoded.to_string(), query);
        assert_eq!(load_data.to_string(), values);
    }

    #[test]
    fn test_commandable() {
        let load_data: &'static str = r#"[
{"_key":"http://example.org/","title":"This is test record 1!"},
]"#;
        let query = LoadCommand::new("test".to_string(), load_data.to_string())
            .input_type(InputType::Json)
            .to_command();
        let cli_encoded = "load --table test --input_type \
                           json\n[\n{\"_key\":\"http://example.org/\",\"title\":\"This is test \
                           record 1!\"},\n]";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_post_commandable() {
        let load_data: &'static str = r#"[
{"_key":"http://example.org/","title":"This is test record 1!"},
]"#;
        let (query, values) = LoadCommand::new("test".to_string(), load_data.to_string())
            .input_type(InputType::Json)
            .to_post_command();
        let cli_encoded = "load --table test --input_type json";
        assert_eq!(cli_encoded.to_string(), query);
        assert_eq!(load_data.to_string(), values);
    }
}
