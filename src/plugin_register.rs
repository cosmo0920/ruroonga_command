use super::command::{Command, Query};
use super::command::Command::PluginRegister;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PluginRegisterCommand {
    command: Command,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for PluginRegisterCommand {
    fn default() -> PluginRegisterCommand {
        PluginRegisterCommand {
            command: PluginRegister,
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl PluginRegisterCommand {
    pub fn new(name: String) -> PluginRegisterCommand {
        let default: PluginRegisterCommand = Default::default();
        PluginRegisterCommand {
            command: default.command,
            name: name,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name)];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (PluginRegister, query)
    }
}

impl Queryable for PluginRegisterCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for PluginRegisterCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(PluginRegisterCommand);
request_cancellable!(PluginRegisterCommand);
request_timeoutable!(PluginRegisterCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::PluginRegister;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let plugin_register = PluginRegisterCommand::new("test_plugin".to_string());
        let expected = PluginRegisterCommand {
            command: PluginRegister,
            name: "test_plugin".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, plugin_register);
    }

    #[test]
    fn test_build() {
        let actual = PluginRegisterCommand::new("test_plugin".to_string()).build();
        let expected_query: Query = vec![("name".to_string(), "test_plugin".to_string())];
        let expected = (PluginRegister, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = PluginRegisterCommand::new("test_plugin".to_string()).to_query();
        let url_encoded = "/d/plugin_register?name=test_plugin";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = PluginRegisterCommand::new("test_plugin".to_string()).to_command();
        let cli_encoded = "plugin_register --name test_plugin";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = PluginRegisterCommand {
            command: PluginRegister,
            name: "test_plugin".to_string(),
            arguments: arg.clone(),
        };
        let query = PluginRegisterCommand::new("test_plugin".to_string());
        unsafe {
            let extended = query.set_arguments(arg.clone());
            assert_eq!(expected, extended);
        }
    }
}
