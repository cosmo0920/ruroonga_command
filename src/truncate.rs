use super::command::{Command, Query};
use super::command::Command::Truncate;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TruncateCommand {
    command: Command,
    target_name: String,
    arguments: HashMap<String, String>,
}

impl Default for TruncateCommand {
    fn default() -> TruncateCommand {
        TruncateCommand {
            command: Truncate,
            target_name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl TruncateCommand {
    pub fn new(target_name: String) -> TruncateCommand {
        let default: TruncateCommand = Default::default();
        TruncateCommand {
            command: default.command,
            target_name: target_name,
            arguments: default.arguments,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("target_name".to_string(), self.target_name)];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Truncate, query)
    }
}

impl Queryable for TruncateCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for TruncateCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(TruncateCommand);
request_cancellable!(TruncateCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Truncate;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let truncate = TruncateCommand::new("truncated".to_string());
        let expected = TruncateCommand {
            command: Truncate,
            target_name: "truncated".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, truncate);
    }

    #[test]
    fn test_build() {
        let actual = TruncateCommand::new("truncated".to_string()).build();
        let expected_query: Query = vec![("target_name".to_string(), "truncated".to_string())];
        let expected = (Truncate, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = TruncateCommand::new("truncating".to_string()).to_query();
        let url_encoded = "/d/truncate?target_name=truncating";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = TruncateCommand::new("burdened".to_string()).to_command();
        let cli_encoded = "truncate --target_name burdened";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = TruncateCommand {
            command: Truncate,
            target_name: "deleting".to_string(),
            arguments: arg.clone(),
        };
        let query = TruncateCommand::new("deleting".to_string());
        unsafe {
            let extended = query.set_arguments(arg.clone());
            assert_eq!(expected, extended);
        }
    }
}
