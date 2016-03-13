use super::command::{Command, Query};
use super::command::Command::Extension;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExtensionCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for ExtensionCommand {
    fn default() -> ExtensionCommand {
        ExtensionCommand {
            command: Extension("".to_string()),
            arguments: HashMap::new(),
        }
    }
}

impl ExtensionCommand {
    pub fn new(command: String) -> ExtensionCommand {
        let default: ExtensionCommand = Default::default();
        ExtensionCommand {
            command: Extension(command),
            arguments: default.arguments,
        }
    }

    pub fn set_arguments(mut self, arguments: HashMap<String, String>) -> ExtensionCommand {
        self.arguments = arguments;
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (self.command, query)
    }
}

impl Queryable for ExtensionCommand {
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
    use command::Command::Extension;
    use std::collections::HashMap;
    use queryable::Queryable;

    #[test]
    fn test_new() {
        let vanilla_extension = ExtensionCommand::new("Added".to_string());
        let expected = ExtensionCommand {
            command: Extension("Added".to_string()),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_extension);
    }

    #[test]
    fn test_set_arguments() {
        let mut arg = HashMap::new();
        arg.insert("test".to_string(), "element".to_string());
        let extension = ExtensionCommand::new("Added".to_string()).
            set_arguments(arg.clone());
        let expected = ExtensionCommand {
            command: Extension("Added".to_string()),
            arguments: arg,
        };
        assert_eq!(expected, extension);
    }

    #[test]
    fn test_build() {
        let mut arg = HashMap::new();
        arg.insert("test".to_string(), "element".to_string());
        let extension = ExtensionCommand::new("Added".to_string()).
            set_arguments(arg.clone()).build();
        let expected_query: Query =
            vec![("test".to_string(), "element".to_string())];
        let expected = (Extension("Added".to_string()), expected_query);
        assert_eq!(expected, extension);
    }

    #[test]
    fn test_queryable() {
        let mut arg = HashMap::new();
        arg.insert("test".to_string(), "element".to_string());
        let extension = ExtensionCommand::new("added".to_string()).
            set_arguments(arg.clone()).to_query();
        let expected = "/d/added?test=element";
        assert_eq!(expected, extension);
    }
}