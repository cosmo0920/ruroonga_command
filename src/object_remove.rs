use command::{Command, Query};
use command::Command::ObjectRemove;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectRemoveCommand {
    command: Command,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for ObjectRemoveCommand {
    fn default() -> ObjectRemoveCommand {
        ObjectRemoveCommand {
            command: ObjectRemove,
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl ObjectRemoveCommand {
    pub fn new(name: String) -> ObjectRemoveCommand {
        ObjectRemoveCommand { name: name, ..ObjectRemoveCommand::default() }
    }

    pub fn force(mut self, force: bool) -> ObjectRemoveCommand {
        let flag = if force { "yes" } else { "no" };
        self.arguments.insert("force".to_string(), flag.to_string());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ObjectRemove, query)
    }
}

impl Queryable for ObjectRemoveCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ObjectRemoveCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ObjectRemoveCommand);
request_cancellable!(ObjectRemoveCommand);
request_timeoutable!(ObjectRemoveCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::ObjectRemove;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let object = "object".to_string();
        let object_remove = ObjectRemoveCommand::new(object.to_owned());
        let expected = ObjectRemoveCommand {
            command: ObjectRemove,
            name: object.to_owned(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, object_remove);
    }

    #[test]
    fn test_cache() {
        let select_yes = ObjectRemoveCommand::new("test".to_string()).force(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("force".to_string(), "yes".to_string());
        let expected_yes = ObjectRemoveCommand {
            command: ObjectRemove,
            name: "test".to_string(),
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, select_yes);
        let select_no = ObjectRemoveCommand::new("test".to_string()).force(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("force".to_string(), "no".to_string());
        let expected_no = ObjectRemoveCommand {
            command: ObjectRemove,
            name: "test".to_string(),
            arguments: arg_no,
        };
        assert_eq!(expected_no, select_no);
    }

    #[test]
    fn test_build() {
        let object = "object".to_string();
        let actual = ObjectRemoveCommand::new(object.to_owned()).build();
        let expected_query: Query = vec![("name".to_string(), object.to_owned())];
        let expected = (ObjectRemove, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let object = "object".to_string();
        let query = ObjectRemoveCommand::new(object).to_query();
        let url_encoded = "/d/object_remove?name=object";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let object = "object".to_string();
        let query = ObjectRemoveCommand::new(object).to_command();
        let cli_encoded = "object_remove --name object";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let object = "object".to_string();
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ObjectRemoveCommand {
            command: ObjectRemove,
            name: object.to_owned(),
            arguments: arg.to_owned(),
        };
        let query = ObjectRemoveCommand::new(object.to_owned());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
