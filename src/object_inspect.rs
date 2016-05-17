use super::command::{Command, Query};
use super::command::Command::ObjectInspect;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectInspectCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for ObjectInspectCommand {
    fn default() -> ObjectInspectCommand {
        ObjectInspectCommand {
            command: ObjectInspect,
            arguments: HashMap::new(),
        }
    }
}

impl ObjectInspectCommand {
    pub fn new() -> ObjectInspectCommand {
        let default: ObjectInspectCommand = Default::default();
        ObjectInspectCommand {
            command: default.command,
            arguments: default.arguments,
        }
    }

    pub fn name(mut self, name: String) -> ObjectInspectCommand {
        self.arguments.insert("name".to_string(), name.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ObjectInspect, query)
    }
}

impl Queryable for ObjectInspectCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ObjectInspectCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ObjectInspectCommand);
request_cancellable!(ObjectInspectCommand);
request_timeoutable!(ObjectInspectCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::ObjectInspect;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let object_inspect = ObjectInspectCommand::new();
        let expected = ObjectInspectCommand {
            command: ObjectInspect,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, object_inspect);
    }

    #[test]
    fn test_name() {
        let actual = ObjectInspectCommand::new().name("object".to_string()).build();
        let expected_query: Query = vec![("name".to_string(), "object".to_string())];
        let expected = (ObjectInspect, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_build() {
        let actual = ObjectInspectCommand::new().name("object".to_string()).build();
        let expected_query: Query = vec![("name".to_string(), "object".to_string())];
        let expected = (ObjectInspect, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = ObjectInspectCommand::new().name("object".to_string()).to_query();
        let url_encoded = "/d/object_inspect?name=object";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = ObjectInspectCommand::new().name("object".to_string()).to_command();
        let cli_encoded = "object_inspect --name object";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ObjectInspectCommand {
            command: ObjectInspect,
            arguments: arg.to_owned(),
        };
        let query = ObjectInspectCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
