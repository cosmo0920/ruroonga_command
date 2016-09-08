use command::{Command, Query};
use command::Command::LogLevel;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;
use types::log_level_type::LogLevelType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogLevelCommand {
    command: Command,
    level: LogLevelType,
    arguments: HashMap<String, String>,
}

impl Default for LogLevelCommand {
    fn default() -> LogLevelCommand {
        LogLevelCommand {
            command: LogLevel,
            level: LogLevelType::Info,
            arguments: HashMap::new(),
        }
    }
}

impl LogLevelCommand {
    pub fn new(level: LogLevelType) -> LogLevelCommand {
        LogLevelCommand {
            level: level,
            ..LogLevelCommand::default()
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("level".to_string(), format!("{}", self.level))];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (LogLevel, query)
    }
}

impl Queryable for LogLevelCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LogLevelCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(LogLevelCommand);
request_cancellable!(LogLevelCommand);
request_timeoutable!(LogLevelCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::LogLevel;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;
    use types::log_level_type::LogLevelType;

    #[test]
    fn test_new() {
        let log_level = LogLevelCommand::new(LogLevelType::Info);
        let expected = LogLevelCommand {
            command: LogLevel,
            level: LogLevelType::Info,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, log_level);
    }

    #[test]
    fn test_build() {
        let actual = LogLevelCommand::new(LogLevelType::Notice).build();
        let expected_query: Query = vec![("level".to_string(),
                                          format!("{}", LogLevelType::Notice))];
        let expected = (LogLevel, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = LogLevelCommand::new(LogLevelType::Notice).to_query();
        let url_encoded = "/d/log_level?level=notice";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = LogLevelCommand::new(LogLevelType::Notice).to_command();
        let cli_encoded = "log_level --level notice";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = LogLevelCommand {
            command: LogLevel,
            level: LogLevelType::Warn,
            arguments: arg.to_owned(),
        };
        let query = LogLevelCommand::new(LogLevelType::Warn);
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
