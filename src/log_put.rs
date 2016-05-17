use super::command::{Command, Query};
use super::command::Command::LogPut;
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
pub struct LogPutCommand {
    command: Command,
    level: LogLevelType,
    message: String,
    arguments: HashMap<String, String>,
}

impl Default for LogPutCommand {
    fn default() -> LogPutCommand {
        LogPutCommand {
            command: LogPut,
            level: LogLevelType::Info,
            message: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl LogPutCommand {
    pub fn new(level: LogLevelType, message: String) -> LogPutCommand {
        let default: LogPutCommand = Default::default();
        LogPutCommand {
            command: default.command,
            level: level,
            message: message,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("level".to_string(), self.level.to_string()),
                                    ("message".to_string(), self.message.to_string())];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (LogPut, query)
    }
}

impl Queryable for LogPutCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LogPutCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(LogPutCommand);
request_cancellable!(LogPutCommand);
request_timeoutable!(LogPutCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::LogPut;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;
    use types::log_level_type::LogLevelType;

    #[test]
    fn test_new() {
        let log_put = LogPutCommand::new(LogLevelType::Info, "Hi, Ruroonga!".to_string());
        let expected = LogPutCommand {
            command: LogPut,
            level: LogLevelType::Info,
            message: "Hi, Ruroonga!".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, log_put);
    }

    #[test]
    fn test_build() {
        let actual = LogPutCommand::new(LogLevelType::Notice, "Hi, Ruroonga!".to_string()).build();
        let expected_query: Query = vec![("level".to_string(),
                                          format!("{}", LogLevelType::Notice)),
                                         ("message".to_string(), "Hi, Ruroonga!".to_string())];
        let expected = (LogPut, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = LogPutCommand::new(LogLevelType::Notice, "Hi, Ruroonga!".to_string())
                        .to_query();
        let url_encoded = "/d/log_put?level=notice&message=Hi%2C+Ruroonga%21";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = LogPutCommand::new(LogLevelType::Notice, "Hi, Ruroonga!".to_string())
                        .to_command();
        let cli_encoded = "log_put --level notice --message Hi, Ruroonga!";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = LogPutCommand {
            command: LogPut,
            level: LogLevelType::Warn,
            message: "Hi, Ruroonga!".to_string(),
            arguments: arg.to_owned(),
        };
        let query = LogPutCommand::new(LogLevelType::Warn, "Hi, Ruroonga!".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
