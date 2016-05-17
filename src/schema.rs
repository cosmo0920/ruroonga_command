use super::command::{Command, Query};
use super::command::Command::Schema;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchemaCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for SchemaCommand {
    fn default() -> SchemaCommand {
        SchemaCommand {
            command: Schema,
            arguments: HashMap::new(),
        }
    }
}

impl SchemaCommand {
    pub fn new() -> SchemaCommand {
        Default::default()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (self.command, query)
    }
}

impl Queryable for SchemaCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for SchemaCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

request_cancellable!(SchemaCommand);
request_timeoutable!(SchemaCommand);

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use command::Command::Schema;
    use queryable::Queryable;
    use commandable::Commandable;

    #[test]
    fn test_new() {
        let schema = SchemaCommand::new();
        let expected = SchemaCommand {
            command: Schema,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, schema);
    }

    #[test]
    fn test_build() {
        let actual = SchemaCommand::new().build();
        let expected = (Schema, vec![]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = SchemaCommand::new().to_query();
        let url_encoded = "/d/schema?";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = SchemaCommand::new().to_command();
        let cli_encoded = "schema";
        assert_eq!(cli_encoded.to_string(), query);
    }
}
