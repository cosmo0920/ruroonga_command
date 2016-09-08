use command::{Command, Query};
use command::Command::ObjectExist;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectExistCommand {
    command: Command,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for ObjectExistCommand {
    fn default() -> ObjectExistCommand {
        ObjectExistCommand {
            command: ObjectExist,
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl ObjectExistCommand {
    pub fn new(name: String) -> ObjectExistCommand {
        ObjectExistCommand { name: name, ..ObjectExistCommand::default() }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ObjectExist, query)
    }
}

impl Queryable for ObjectExistCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ObjectExistCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ObjectExistCommand);
request_cancellable!(ObjectExistCommand);
request_timeoutable!(ObjectExistCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::ObjectExist;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let object = "object".to_string();
        let object_exist = ObjectExistCommand::new(object.to_owned());
        let expected = ObjectExistCommand {
            command: ObjectExist,
            name: object.to_owned(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, object_exist);
    }

    #[test]
    fn test_build() {
        let object = "object".to_string();
        let actual = ObjectExistCommand::new(object.to_owned()).build();
        let expected_query: Query = vec![("name".to_string(), object.to_owned())];
        let expected = (ObjectExist, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let object = "object".to_string();
        let query = ObjectExistCommand::new(object).to_query();
        let url_encoded = "/d/object_exist?name=object";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let object = "object".to_string();
        let query = ObjectExistCommand::new(object).to_command();
        let cli_encoded = "object_exist --name object";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let object = "object".to_string();
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ObjectExistCommand {
            command: ObjectExist,
            name: object.to_owned(),
            arguments: arg.to_owned(),
        };
        let query = ObjectExistCommand::new(object.to_owned());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
