use url::form_urlencoded;
use command::{Command, Query};
use command::Command::Dump;

#[derive(Clone, Debug)]
pub struct CommandQuery {
    command: Command,
    arguments: Query,
    prefix: String,
}

impl Default for CommandQuery {
    fn default() -> CommandQuery {
        CommandQuery {
            command: Dump,
            arguments: vec![],
            prefix: "/d".to_string(),
        }
    }
}

impl CommandQuery {
    pub fn new(command: Command, query: Query) -> CommandQuery {
        let default: CommandQuery = Default::default();
        CommandQuery {
            command: command,
            arguments: query,
            prefix: default.prefix,
        }
    }

    /// Get vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn get_command(&mut self) -> Command {
        self.command.clone()
    }

    /// Set vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn set_argument(&mut self, arguments: Query) {
        self.arguments = arguments
    }

    #[doc(hidden)]
    // get HTTP URI prefix. default: /d
    // This function is mainly provided for internal usage.
    pub fn get_prefix(&mut self) -> String {
        self.prefix.clone()
    }

    #[doc(hidden)]
    // set HTTP URI prefix. This function is provided for advanced user.
    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix
    }

    /// Create url encoded command query.
    ///
    /// `vec![("key","value")]` interprets to `"key=value"`.
    /// And two or more value pair are concatinate with `&`.
    pub fn make_query(&mut self) -> String {
        form_urlencoded::serialize(self.arguments.clone().into_iter())
    }

    ///
    /// Create Groonga HTTP server query URL.
    pub fn encode(&mut self) -> String {
        format!("{}/{}?{}",
                self.get_prefix(),
                self.get_command(),
                self.make_query())
    }
}

#[cfg(test)]
mod test {
    use super::CommandQuery;
    use select::SelectCommand;
    use load::{LoadCommand, InputType};
    use status::StatusCommand;
    use dump::DumpCommand;
    use delete::DeleteCommand;

    #[test]
    fn test_select() {
        let (command, query) = SelectCommand::new("Test".to_string())
            .filter("output_column @ \"type_safe\"".to_string()).build();
        let mut command = CommandQuery::new(command, query);
        let url_encoded = "/d/select?table=Test&filter=%27output_column+%40+%22type_safe%22%27";
        assert_eq!(url_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_load() {
        let load_data: &'static str = r#"[
{"_key":"http://example.org/","title":"This is test record 1!"},
]"#;
        let (command, query, values) =
            LoadCommand::new("test".to_string(), load_data.to_string())
            .input_type(InputType::JSON).build();
        let mut command = CommandQuery::new(command, query);
        let url_encoded = "/d/load?table=test&input_type=json";
        assert_eq!(url_encoded.to_string(), command.encode());
        assert_eq!(load_data.to_string(), values);
    }

    #[test]
    fn test_status() {
        let (command, query) = StatusCommand::new().build();
        let mut command = CommandQuery::new(command, query);
        let url_encoded = "/d/status?";
        assert_eq!(url_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_dump() {
        let (command, query) = DumpCommand::new()
            .tables(vec!["Books".to_string(), "Categories".to_string()])
            .build();
        let mut command = CommandQuery::new(command, query);
        let url_encoded = "/d/dump?tables=Books%2CCategories";
        assert_eq!(url_encoded.to_string(), command.encode());
    }

    #[test]
    fn test_delete() {
        let (command, query) = DeleteCommand::new("Books".to_string())
            .filter("author == unknown".to_string())
            .build();
        let mut command = CommandQuery::new(command, query);
        let url_encoded = "/d/delete?table=Books&filter=%27author+%3D%3D+unknown%27";
        assert_eq!(url_encoded.to_string(), command.encode());
    }
}
