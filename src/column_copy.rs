use command::{Command, Query};
use command::Command::ColumnCopy;
use std::collections::HashMap;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColumnCopyCommand {
    command: Command,
    from_table: String,
    from_name: String,
    to_table: String,
    to_name: String,
    arguments: HashMap<String, String>,
}

impl Default for ColumnCopyCommand {
    fn default() -> ColumnCopyCommand {
        ColumnCopyCommand {
            command: ColumnCopy,
            from_table: "".to_string(),
            from_name: "".to_string(),
            to_table: "".to_string(),
            to_name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl ColumnCopyCommand {
    pub fn new(from_table: String,
               from_name: String,
               to_table: String,
               to_name: String)
               -> ColumnCopyCommand {
        ColumnCopyCommand {
            from_table: from_table,
            from_name: from_name,
            to_table: to_table,
            to_name: to_name,
            ..ColumnCopyCommand::default()
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("from_table".to_string(), self.from_table),
                                    ("from_name".to_string(), self.from_name),
                                    ("to_table".to_string(), self.to_table),
                                    ("to_name".to_string(), self.to_name)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (ColumnCopy, query)
    }
}

impl Queryable for ColumnCopyCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for ColumnCopyCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(ColumnCopyCommand);
request_cancellable!(ColumnCopyCommand);
request_timeoutable!(ColumnCopyCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Command::ColumnCopy;
    use std::collections::HashMap;
    use command::Query;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let vanilla_column_create = ColumnCopyCommand::new("Test".to_string(),
                                                           "element".to_string(),
                                                           "NewTest".to_string(),
                                                           "new_element".to_string());
        let expected = ColumnCopyCommand {
            command: ColumnCopy,
            from_table: "Test".to_string(),
            from_name: "element".to_string(),
            to_table: "NewTest".to_string(),
            to_name: "new_element".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_column_create);
    }

    #[test]
    fn test_build() {
        let actual = ColumnCopyCommand::new("Test".to_string(),
                                            "element".to_string(),
                                            "NewTest".to_string(),
                                            "new_element".to_string())
            .build();
        let expected_query: Query = vec![("from_table".to_string(), "Test".to_string()),
                                         ("from_name".to_string(), "element".to_string()),
                                         ("to_table".to_string(), "NewTest".to_string()),
                                         ("to_name".to_string(), "new_element".to_string())];
        let expected = (ColumnCopy, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let actual = ColumnCopyCommand::new("Test".to_string(),
                                            "element".to_string(),
                                            "NewTest".to_string(),
                                            "new_element".to_string())
            .to_query();
        let expected = "/d/column_copy?from_table=Test&from_name=element&to_table=NewTest&to_name=\
                        new_element";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_commandable() {
        let actual = ColumnCopyCommand::new("Test".to_string(),
                                            "element".to_string(),
                                            "NewTest".to_string(),
                                            "new_element".to_string())
            .to_command();
        let expected = "column_copy --from_table Test --from_name element --to_table NewTest \
                        --to_name new_element";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = ColumnCopyCommand {
            command: ColumnCopy,
            from_table: "Test".to_string(),
            from_name: "element".to_string(),
            to_table: "NewTest".to_string(),
            to_name: "new_element".to_string(),
            arguments: arg.to_owned(),
        };
        let query = ColumnCopyCommand::new("Test".to_string(),
                                           "element".to_string(),
                                           "NewTest".to_string(),
                                           "new_element".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
