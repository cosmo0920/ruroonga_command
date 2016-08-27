use command::{Command, Query};
use command::Command::Dump;
use std::collections::HashMap;
use util;
use command_query::CommandQuery;
use queryable::Queryable;
use command_line::CommandLine;
use commandable::Commandable;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DumpCommand {
    command: Command,
    arguments: HashMap<String, String>,
}

impl Default for DumpCommand {
    fn default() -> DumpCommand {
        DumpCommand {
            command: Dump,
            arguments: HashMap::new(),
        }
    }
}

impl DumpCommand {
    pub fn new() -> DumpCommand {
        Default::default()
    }

    pub fn tables(mut self, tables: Vec<String>) -> DumpCommand {
        let string = util::split_values_vec(tables);
        self.arguments.insert("tables".to_string(), string.to_owned());
        self
    }

    pub fn dump_plugins(mut self, include: bool) -> DumpCommand {
        let flag = if include { "yes" } else { "no" };
        self.arguments.insert("dump_plugins".to_string(), flag.to_string());
        self
    }

    pub fn dump_schema(mut self, include: bool) -> DumpCommand {
        let flag = if include { "yes" } else { "no" };
        self.arguments.insert("dump_schema".to_string(), flag.to_string());
        self
    }

    pub fn dump_records(mut self, include: bool) -> DumpCommand {
        let flag = if include { "yes" } else { "no" };
        self.arguments.insert("dump_records".to_string(), flag.to_string());
        self
    }

    pub fn dump_indexes(mut self, include: bool) -> DumpCommand {
        let flag = if include { "yes" } else { "no" };
        self.arguments.insert("dump_indexes".to_string(), flag.to_string());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Dump, query)
    }
}

impl Queryable for DumpCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for DumpCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

extendable!(DumpCommand);
request_cancellable!(DumpCommand);
request_timeoutable!(DumpCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Dump;
    use std::collections::HashMap;
    use queryable::Queryable;
    use commandable::Commandable;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let vanilla_dump = DumpCommand::new();
        let expected = DumpCommand {
            command: Dump,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_dump);
    }

    #[test]
    fn test_tables() {
        let dump = DumpCommand::new().tables(vec!["Books".to_string(), "Categories".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("tables".to_string(), "Books,Categories".to_string());
        let expected = DumpCommand {
            command: Dump,
            arguments: arg,
        };
        assert_eq!(expected, dump);
    }

    #[test]
    fn test_dump_plugins() {
        let dump_yes = DumpCommand::new().dump_plugins(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("dump_plugins".to_string(), "yes".to_string());
        let expected_yes = DumpCommand {
            command: Dump,
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, dump_yes);
        let dump_no = DumpCommand::new().dump_plugins(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("dump_plugins".to_string(), "no".to_string());
        let expected_no = DumpCommand {
            command: Dump,
            arguments: arg_no,
        };
        assert_eq!(expected_no, dump_no);
    }

    #[test]
    fn test_dump_schemas() {
        let dump_yes = DumpCommand::new().dump_schema(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("dump_schema".to_string(), "yes".to_string());
        let expected_yes = DumpCommand {
            command: Dump,
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, dump_yes);
        let dump_no = DumpCommand::new().dump_schema(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("dump_schema".to_string(), "no".to_string());
        let expected_no = DumpCommand {
            command: Dump,
            arguments: arg_no,
        };
        assert_eq!(expected_no, dump_no);
    }

    #[test]
    fn test_dump_records() {
        let dump_yes = DumpCommand::new().dump_records(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("dump_records".to_string(), "yes".to_string());
        let expected_yes = DumpCommand {
            command: Dump,
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, dump_yes);
        let dump_no = DumpCommand::new().dump_records(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("dump_records".to_string(), "no".to_string());
        let expected_no = DumpCommand {
            command: Dump,
            arguments: arg_no,
        };
        assert_eq!(expected_no, dump_no);
    }

    #[test]
    fn test_dump_indexes() {
        let dump_yes = DumpCommand::new().dump_indexes(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("dump_indexes".to_string(), "yes".to_string());
        let expected_yes = DumpCommand {
            command: Dump,
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, dump_yes);
        let dump_no = DumpCommand::new().dump_indexes(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("dump_indexes".to_string(), "no".to_string());
        let expected_no = DumpCommand {
            command: Dump,
            arguments: arg_no,
        };
        assert_eq!(expected_no, dump_no);
    }

    #[test]
    fn test_build() {
        let actual = DumpCommand::new()
            .tables(vec!["Books".to_string(), "Categories".to_string()])
            .build();
        let expected_query: Query = vec![("tables".to_string(), "Books,Categories".to_string())];
        let expected = (Dump, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = DumpCommand::new()
            .tables(vec!["Books".to_string(), "Categories".to_string()])
            .to_query();
        let url_encoded = "/d/dump?tables=Books%2CCategories";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = DumpCommand::new()
            .tables(vec!["Books".to_string(), "Categories".to_string()])
            .to_command();
        let cli_encoded = "dump --tables Books,Categories";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = DumpCommand {
            command: Dump,
            arguments: arg.to_owned(),
        };
        let query = DumpCommand::new();
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
