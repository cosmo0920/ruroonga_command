use super::command::{Command, Query};
use super::command::Command::PluginUnregister;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PluginUnregisterCommand {
    command: Command,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for PluginUnregisterCommand {
    fn default() -> PluginUnregisterCommand {
        PluginUnregisterCommand {
            command: PluginUnregister,
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl PluginUnregisterCommand {
    pub fn new(name: String) -> PluginUnregisterCommand {
        let default: PluginUnregisterCommand = Default::default();
        PluginUnregisterCommand {
            command: default.command,
            name: name,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (PluginUnregister, query)
    }
}

impl Queryable for PluginUnregisterCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for PluginUnregisterCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(PluginUnregisterCommand);
request_cancellable!(PluginUnregisterCommand);
request_timeoutable!(PluginUnregisterCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::PluginUnregister;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let plugin_unregister = PluginUnregisterCommand::new("test_plugin".to_string());
        let expected = PluginUnregisterCommand {
            command: PluginUnregister,
            name: "test_plugin".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, plugin_unregister);
    }

    #[test]
    fn test_build() {
        let actual = PluginUnregisterCommand::new("test_plugin".to_string()).build();
        let expected_query: Query = vec![("name".to_string(), "test_plugin".to_string())];
        let expected = (PluginUnregister, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = PluginUnregisterCommand::new("test_plugin".to_string()).to_query();
        let url_encoded = "/d/plugin_unregister?name=test_plugin";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = PluginUnregisterCommand::new("test_plugin".to_string()).to_command();
        let cli_encoded = "plugin_unregister --name test_plugin";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = PluginUnregisterCommand {
            command: PluginUnregister,
            name: "test_plugin".to_string(),
            arguments: arg.to_owned(),
        };
        let query = PluginUnregisterCommand::new("test_plugin".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
