use super::command::{Command, Query};
use super::command::Command::IoFlush;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IoFlushCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for IoFlushCommand {
    fn default() -> IoFlushCommand {
        IoFlushCommand {
            command: IoFlush,
            arguments: HashMap::new(),
        }
    }
}

impl IoFlushCommand {
    pub fn new() -> IoFlushCommand {
        Default::default()
    }

    pub fn target_name(mut self, target_name: String) -> IoFlushCommand {
        self.arguments.insert("target_name".to_string(), target_name);
        self
    }

    pub fn recursive(mut self, recursive: bool) -> IoFlushCommand {
        let flag = if recursive {
            "yes"
        } else {
            "no"
        };
        self.arguments.insert("recursive".to_string(), flag.to_string());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (IoFlush, query)
    }
}

impl Queryable for IoFlushCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for IoFlushCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(IoFlushCommand);
request_cancellable!(IoFlushCommand);
request_timeoutable!(IoFlushCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::IoFlush;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let io_flush = IoFlushCommand::new();
        let expected = IoFlushCommand {
            command: IoFlush,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, io_flush);
    }

    #[test]
    fn test_target_name() {
        let io_flush = IoFlushCommand::new().target_name("Table".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("target_name".to_string(), "Table".to_string());
        let expected = IoFlushCommand {
            command: IoFlush,
            arguments: arg,
        };
        assert_eq!(expected, io_flush);
    }

    #[test]
    fn test_recursive() {
        let recursive_yes = IoFlushCommand::new().recursive(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("recursive".to_string(), "yes".to_string());
        let expected_yes = IoFlushCommand {
            command: IoFlush,
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, recursive_yes);
        let recursive_no = IoFlushCommand::new().recursive(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("recursive".to_string(), "no".to_string());
        let expected_no = IoFlushCommand {
            command: IoFlush,
            arguments: arg_no,
        };
        assert_eq!(expected_no, recursive_no);
    }

    #[test]
    fn test_build() {
        let actual = IoFlushCommand::new().build();
        let expected_query: Query = vec![];
        let expected = (IoFlush, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = IoFlushCommand::new().target_name("Table".to_string()).to_query();
        let url_encoded = "/d/io_flush?target_name=Table";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = IoFlushCommand::new().target_name("Table".to_string()).to_command();
        let cli_encoded = "io_flush --target_name Table";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = IoFlushCommand {
            command: IoFlush,
            arguments: arg.to_owned(),
        };
        let query = IoFlushCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
