use super::command::{Command, Query};
use super::command::Command::Shutdown;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;
use types::shutdown_mode_type::ShutdownModeType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShutdownCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for ShutdownCommand {
    fn default() -> ShutdownCommand {
        ShutdownCommand {
            command: Shutdown,
            arguments: HashMap::new(),
        }
    }
}

impl ShutdownCommand {
    pub fn new() -> ShutdownCommand {
        Default::default()
    }

    pub fn mode(mut self, mode: ShutdownModeType) -> ShutdownCommand {
        let mode = format!("{}", mode);
        self.arguments.insert("mode".to_string(), mode);
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Shutdown, query)
    }
}

impl Queryable for ShutdownCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ShutdownCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ShutdownCommand);
request_cancellable!(ShutdownCommand);
request_timeoutable!(ShutdownCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Shutdown;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;
    use types::shutdown_mode_type::ShutdownModeType;

    #[test]
    fn test_new() {
        let shutdown = ShutdownCommand::new();
        let expected = ShutdownCommand {
            command: Shutdown,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, shutdown);
    }

    #[test]
    fn test_mode() {
        let shutdown = ShutdownCommand::new().mode(ShutdownModeType::Immediate);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("mode".to_string(), "immediate".to_string());
        let expected = ShutdownCommand {
            command: Shutdown,
            arguments: arg,
        };
        assert_eq!(expected, shutdown);
    }

    #[test]
    fn test_build() {
        let actual = ShutdownCommand::new().build();
        let expected_query: Query = vec![];
        let expected = (Shutdown, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = ShutdownCommand::new().mode(ShutdownModeType::Immediate).to_query();
        let url_encoded = "/d/shutdown?mode=immediate";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = ShutdownCommand::new().mode(ShutdownModeType::Graceful).to_command();
        let cli_encoded = "shutdown --mode graceful";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ShutdownCommand {
            command: Shutdown,
            arguments: arg.to_owned(),
        };
        let query = ShutdownCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
