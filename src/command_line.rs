use command::{Command, Query};
use commandlinable::Escape;

#[derive(Clone, Debug)]
pub struct CommandLine {
    command: Command,
    arguments: Query,
}

impl CommandLine {
    pub fn new(command: Command, query: Query) -> CommandLine {
        CommandLine {
            command: command,
            arguments: query,
        }
    }

    /// Get command enum constant to construct command line query.
    pub fn get_command(&mut self) -> Command {
        self.command.clone()
    }

    /// Get vectorize `("key", "value")` pairs to construct command line query.
    pub fn get_argument(&mut self) -> Query {
        self.arguments.clone()
    }

    /// Set vectorize `("key", "value")` pairs to construct command line query.
    pub fn set_argument(&mut self, arguments: Query) {
        self.arguments = arguments
    }

    /// Create command line style command query.
    ///
    /// `vec![("key","value")]` interprets to `"--key value"`.
    /// And two or more value pair are concatinate with ` `(a space).
    pub fn make_command(&mut self) -> String {
        let mut output = String::new();
        for query in self.arguments.to_owned() {
            let (key, value) = query;
            if output.len() > 0 {
                output.push_str(" ");
            }
            let format_string = format!("--{} {}", &*key, Escape(&*value));
            output.push_str(format_string.as_str());
        }
        output
    }

    ///
    /// Create Groonga command line style command.
    pub fn encode(&mut self) -> String {
        format!("{} {}", self.get_command(), self.make_command())
            .trim()
            .clone()
    }
}

#[cfg(test)]
mod test {
    use super::CommandLine;
    use select::SelectCommand;
    use load::{LoadCommand, InputType};
    use status::StatusCommand;
    use dump::DumpCommand;
    use delete::DeleteCommand;
    use table_create::TableCreateCommand;
    use types::table_flag_type::TableFlagType;
    use column_create::ColumnCreateCommand;
    use types::data_type::DataType;

    #[test]
    fn test_select() {
        let (command, query) = SelectCommand::new("Test".to_string())
                                   .filter("output_column @ \"type_safe\"".to_string())
                                   .build();
        let mut command = CommandLine::new(command, query);
        let cli_encoded = "select --table Test --filter \'output_column @ \"type_safe\"\'";
        assert_eq!(cli_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_load() {
        let load_data: &'static str = r#"[
{"_key":"http://example.org/","title":"This is test record 1!"},
]"#;
        let (command, query, values) = LoadCommand::new("Test".to_string(), load_data.to_string())
                                           .input_type(InputType::Json)
                                           .build();
        let mut command = CommandLine::new(command, query);
        let cli_encoded = "load --table Test --input_type json";
        assert_eq!(cli_encoded.to_string(), command.encode());
        assert_eq!(load_data.to_string(), values);
    }

    #[test]
    fn test_status() {
        let (command, query) = StatusCommand::new().build();
        let mut command = CommandLine::new(command, query);
        let cli_encoded = "status";
        assert_eq!(cli_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_dump() {
        let (command, query) = DumpCommand::new()
                                   .tables(vec!["Books".to_string(), "Categories".to_string()])
                                   .build();
        let mut command = CommandLine::new(command, query);
        let cli_encoded = "dump --tables Books,Categories";
        assert_eq!(cli_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_delete() {
        let (command, query) = DeleteCommand::new("Books".to_string())
                                   .filter("author == unknown".to_string())
                                   .build();
        let mut command = CommandLine::new(command, query);
        let cli_encoded = "delete --table Books --filter \'author == unknown\'";
        assert_eq!(cli_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_table_create() {
        let (command, query) = TableCreateCommand::new("Test".to_string())
                                   .flags(vec![(TableFlagType::PatKey),
                                               (TableFlagType::KeyWithSIS)])
                                   .build();
        let mut command = CommandLine::new(command, query);
        let url_encoded = "table_create --name Test --flags TABLE_PAT_KEY|KEY_WITH_SIS";
        assert_eq!(url_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_column_create() {
        let (command, query) = ColumnCreateCommand::new("Test".to_string(), "element".to_string())
                                   .column_type(DataType::LongText)
                                   .build();
        let mut command = CommandLine::new(command, query);
        let cli_encoded = "column_create --table Test --name element --type LongText";
        assert_eq!(cli_encoded.to_string(), command.encode());
    }
}
