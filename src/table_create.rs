use super::command::{Command, Query};
use super::command::Command::TableCreate;
use std::collections::HashMap;
use types::table_flag_type::TableFlagType;
use types::data_type::DataType;
use types::tokenizer_type::TokenizerType;
use types::normalizer_type::NormalizerType;
use types::token_filters_type::TokenFiltersType;
use util;

pub type LoadValues = String;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TableCreateCommand {
    command: Command,
    name: String,
    arguments: HashMap<String, String>,
}

impl Default for TableCreateCommand {
    fn default() -> TableCreateCommand {
        TableCreateCommand {
            command: TableCreate,
            name: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl TableCreateCommand {
    pub fn new(name: String) -> TableCreateCommand {
        let default: TableCreateCommand = Default::default();
        TableCreateCommand {
            command: default.command,
            name: name,
            arguments: default.arguments,
        }
    }

    pub fn flags(mut self, flags: Vec<TableFlagType>) -> TableCreateCommand {
        let string = util::split_flags_vec(flags);
        self.arguments.insert("flags".to_string(), string.clone());
        self
    }

    pub fn key_type(mut self, key_type: DataType) -> TableCreateCommand {
        let string = format!("{}", key_type);
        self.arguments.insert("key_type".to_string(), string.clone());
        self
    }

    pub fn default_tokenizer(mut self, tokenizer: TokenizerType) -> TableCreateCommand {
        let string = format!("{}", tokenizer);
        self.arguments.insert("default_tokenizer".to_string(), string.clone());
        self
    }

    pub fn normalizer(mut self, normalizer: NormalizerType) -> TableCreateCommand {
        let string = format!("{}", normalizer);
        self.arguments.insert("normalizer".to_string(), string.clone());
        self
    }

    pub fn token_filter(mut self, token_filter: TokenFiltersType) -> TableCreateCommand {
        let string = format!("{}", token_filter);
        self.arguments.insert("token_filter".to_string(), string.clone());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("name".to_string(), self.name)];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (TableCreate, query)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use command::Command::TableCreate;
    use std::collections::HashMap;
    use types::table_flag_type::TableFlagType;
    use types::data_type::DataType;
    use types::tokenizer_type::TokenizerType;
    use types::normalizer_type::NormalizerType;
    use types::token_filters_type::TokenFiltersType;
    use command::Query;

    #[test]
    fn test_new() {
        let vanilla_table_create = TableCreateCommand::new("Test".to_string());
        let expected = TableCreateCommand {
            command: TableCreate,
            name: "Test".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_table_create);
    }

    #[test]
    fn test_flags() {
        let table_create = TableCreateCommand::new("Test".to_string())
            .flags(vec![(TableFlagType::PatKey), (TableFlagType::KeyWithSIS)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("flags".to_string(),
                   "TABLE_PAT_KEY|TABLE_KEY_WITH_SIS".to_string());
        let expected = TableCreateCommand {
            command: TableCreate,
            name: "Test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, table_create);
    }

    #[test]
    fn test_key_type() {
        let table_create = TableCreateCommand::new("Test".to_string())
            .key_type(DataType::TokyoGeoPoint);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("key_type".to_string(), "TokyoGeoPoint".to_string());
        let expected = TableCreateCommand {
            command: TableCreate,
            name: "Test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, table_create);
    }

    #[test]
    fn test_default_tokenizer() {
        let table_create = TableCreateCommand::new("Test".to_string())
            .default_tokenizer(TokenizerType::BigramSplitSymbolAlpha);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("default_tokenizer".to_string(),
                   "TokenBigramSplitSymbolAlpha".to_string());
        let expected = TableCreateCommand {
            command: TableCreate,
            name: "Test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, table_create);
    }

    #[test]
    fn test_normalizer() {
        let table_create = TableCreateCommand::new("Test".to_string())
            .normalizer(NormalizerType::Auto);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("normalizer".to_string(),
                   "NormalizerAuto".to_string());
        let expected = TableCreateCommand {
            command: TableCreate,
            name: "Test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, table_create);
    }

    #[test]
    fn test_token_filter() {
        let table_create = TableCreateCommand::new("Test".to_string())
            .token_filter(TokenFiltersType::StopWord);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("token_filter".to_string(),
                   "TokenFilterStopWord".to_string());
        let expected = TableCreateCommand {
            command: TableCreate,
            name: "Test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, table_create);
    }

    #[test]
    fn test_build() {
        let actual = TableCreateCommand::new("Test".to_string())
            .flags(vec![(TableFlagType::PatKey), (TableFlagType::KeyWithSIS)]).build();
        let expected_query: Query =
            vec![("name".to_string(), "Test".to_string()),
                 ("flags".to_string(),
                  "TABLE_PAT_KEY|TABLE_KEY_WITH_SIS".to_string())];
        let expected = (TableCreate, expected_query);
        assert_eq!(expected, actual);
    }

}
