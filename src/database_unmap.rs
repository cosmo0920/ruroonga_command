use command::{Command, Query};
use command::Command::DatabaseUnmap;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DatabaseUnmapCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for DatabaseUnmapCommand {
    fn default() -> DatabaseUnmapCommand {
        DatabaseUnmapCommand {
            command: DatabaseUnmap,
            arguments: HashMap::new(),
        }
    }
}

impl DatabaseUnmapCommand {
    pub fn new() -> DatabaseUnmapCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (DatabaseUnmap, query)
    }
}

impl Queryable for DatabaseUnmapCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for DatabaseUnmapCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(DatabaseUnmapCommand);
request_cancellable!(DatabaseUnmapCommand);
request_timeoutable!(DatabaseUnmapCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::DatabaseUnmap;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let database_unmap = DatabaseUnmapCommand::new();
        let expected = DatabaseUnmapCommand {
            command: DatabaseUnmap,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, database_unmap);
    }

    #[test]
    fn test_build() {
        let actual = DatabaseUnmapCommand::new().build();
        let expected_query: Query = vec![];
        let expected = (DatabaseUnmap, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = DatabaseUnmapCommand::new().to_query();
        let url_encoded = "/d/database_unmap?";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = DatabaseUnmapCommand::new().to_command();
        let cli_encoded = "database_unmap";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = DatabaseUnmapCommand {
            command: DatabaseUnmap,
            arguments: arg.to_owned(),
        };
        let query = DatabaseUnmapCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
