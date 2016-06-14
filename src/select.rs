use super::command::{Command, Query};
use super::command::Command::Select;
use std::collections::HashMap;
use util;
use types::select_query_flags::QueryFlagsType;
use command_query::CommandQuery;
use queryable::Queryable;
use commandable::Commandable;
use command_line::CommandLine;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};
use selectable::drilldown::Drilldown;
use selectable::drilldownable::Drilldownable;
use selectable::drilldown_builder::DrilldownBuilder;
use selectable::labeled_drilldown::LabeledDrilldown;
use selectable::labeled_drilldownable::LabeledDrilldownable;
use selectable::labeled_drilldown_builder::LabeledDrilldownBuilder;
use selectable::labeled_drilldown_sequencable::LabeledDrilldownSequencable;
use selectable::labeled_drilldown_sequence_builder::LabeledDrilldownSequenceBuilder;
use selectable::staged_columns::StagedColumns;
use selectable::staged_columns_builder::StagedColumnsBuilder;
use selectable::staged_columnable::StagedColumnable;
use std::ops::Add;
use extendable::Extendable;
use request_cancellable::RequestCancellable;
use request_timeoutable::RequestTimeoutable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectCommand {
    command: Command,
    table: String,
    arguments: HashMap<String, String>,
}

impl Default for SelectCommand {
    fn default() -> SelectCommand {
        SelectCommand {
            command: Select,
            table: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl SelectCommand {
    pub fn new(table: String) -> SelectCommand {
        let default: SelectCommand = Default::default();
        SelectCommand {
            command: default.command,
            table: table,
            arguments: default.arguments,
        }
    }

    pub fn filter(mut self, filter: String) -> SelectCommand {
        let encoded = format!("'{}'", filter);
        self.arguments.insert("filter".to_string(), encoded.to_owned());
        self
    }

    pub fn query(mut self, query: String) -> SelectCommand {
        let encoded = format!("'{}'", query);
        self.arguments.insert("query".to_string(), encoded.to_owned());
        self
    }

    pub fn scorer(mut self, scorer: String) -> SelectCommand {
        let encoded = format!("'{}'", scorer);
        self.arguments.insert("scorer".to_string(), encoded.to_owned());
        self
    }

    #[deprecated]
    #[allow(deprecated)]
    pub fn sortby(mut self, targets: Vec<String>) -> SelectCommand {
        let string = util::split_values_vec(targets);
        let encoded = format!("'{}'", string);
        self.arguments.insert("sortby".to_string(), encoded.to_owned());
        self
    }

    pub fn sort_keys(mut self, targets: Vec<String>) -> SelectCommand {
        let string = util::split_values_vec(targets);
        let encoded = format!("'{}'", string);
        self.arguments.insert("sort_keys".to_string(), encoded.to_owned());
        self
    }

    pub fn adjuster(mut self, adjust_expr: String) -> SelectCommand {
        let encoded = format!("'{}'", adjust_expr);
        self.arguments.insert("adjuster".to_string(), encoded.to_owned());
        self
    }

    pub fn match_columns(mut self, columns: Vec<String>) -> SelectCommand {
        let string = util::split_values_vec(columns);
        self.arguments.insert("match_columns".to_string(), string.to_owned());
        self
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> SelectCommand {
        let string = util::split_values_vec(columns);
        self.arguments.insert("output_columns".to_string(), string.to_owned());
        self
    }

    pub fn offset(mut self, offset: i64) -> SelectCommand {
        let offset = format!("{}", offset);
        self.arguments.insert("offset".to_string(), offset.to_owned());
        self
    }

    pub fn limit(mut self, limit: i64) -> SelectCommand {
        let limit = format!("{}", limit);
        self.arguments.insert("limit".to_string(), limit.to_owned());
        self
    }

    pub fn cache(mut self, cache: bool) -> SelectCommand {
        let flag = if cache {
            "yes"
        } else {
            "no"
        };
        self.arguments.insert("cache".to_string(), flag.to_string());
        self
    }

    pub fn match_escalation_threshold(mut self, threshold: i64) -> SelectCommand {
        let string = format!("{}", threshold);
        self.arguments.insert("match_escalation_threshold".to_string(), string.to_owned());
        self
    }

    pub fn query_flags(mut self, flags: Vec<QueryFlagsType>) -> SelectCommand {
        let string = util::split_flags_vec(flags);
        self.arguments.insert("query_flags".to_string(), string.to_owned());
        self
    }

    pub fn query_expander(mut self, synonym: (String, String)) -> SelectCommand {
        let (table, column) = synonym;
        let string = format!("{}.{}", table, column);
        self.arguments.insert("query_expander".to_string(), string.to_owned());
        self
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("table".to_string(), self.table)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Select, query)
    }
}

impl Queryable for SelectCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for SelectCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

impl Fragmentable for SelectCommand {
    fn to_fragment(self) -> (Command, OrderedFragment, QueryFragment) {
        let mut select_fragment = HashMap::new();
        let ordered_fragment = vec![("table".to_string(), self.table)];
        for (key, value) in &self.arguments {
            select_fragment.insert(key.to_owned(), value.to_owned());
        }
        (Select, ordered_fragment, select_fragment)
    }
}

impl Add<Drilldown> for SelectCommand {
    type Output = DrilldownBuilder;

    fn add(self, rhs: Drilldown) -> DrilldownBuilder {
        DrilldownBuilder::new(self, rhs)
    }
}

impl Add<LabeledDrilldown> for SelectCommand {
    type Output = LabeledDrilldownBuilder;

    fn add(self, rhs: LabeledDrilldown) -> LabeledDrilldownBuilder {
        LabeledDrilldownBuilder::new(self, rhs)
    }
}

impl Add<Vec<LabeledDrilldown>> for SelectCommand {
    type Output = LabeledDrilldownSequenceBuilder;

    fn add(self, rhs: Vec<LabeledDrilldown>) -> LabeledDrilldownSequenceBuilder {
        LabeledDrilldownSequenceBuilder::new(self, rhs)
    }
}

impl Add<StagedColumns> for SelectCommand {
    type Output = StagedColumnsBuilder;

    fn add(self, rhs: StagedColumns) -> StagedColumnsBuilder {
        StagedColumnsBuilder::new(self, rhs)
    }
}

impl Drilldownable for SelectCommand {
    fn with_drilldown(self, rhs: Drilldown) -> DrilldownBuilder {
        DrilldownBuilder::new(self, rhs)
    }
}

impl LabeledDrilldownable for SelectCommand {
    fn with_labeled_drilldown(self, rhs: LabeledDrilldown) -> LabeledDrilldownBuilder {
        LabeledDrilldownBuilder::new(self, rhs)
    }
}


impl LabeledDrilldownSequencable for SelectCommand {
    fn with_labeled_drilldown_sequence(self,
                                       rhs: Vec<LabeledDrilldown>)
                                       -> LabeledDrilldownSequenceBuilder {
        LabeledDrilldownSequenceBuilder::new(self, rhs)
    }
}

impl StagedColumnable for SelectCommand {
    fn with_staged_columns(self, rhs: StagedColumns) -> StagedColumnsBuilder {
        StagedColumnsBuilder::new(self, rhs)
    }
}

extendable!(SelectCommand);
request_cancellable!(SelectCommand);
request_timeoutable!(SelectCommand);

#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::Select;
    use std::collections::HashMap;
    use types::select_query_flags::QueryFlagsType;
    use queryable::Queryable;
    use commandable::Commandable;
    use selectable::drilldown::Drilldown;
    use selectable::drilldownable::Drilldownable;
    use selectable::drilldown_builder::DrilldownBuilder;
    use selectable::labeled_drilldown::LabeledDrilldown;
    use selectable::labeled_drilldownable::LabeledDrilldownable;
    use selectable::labeled_drilldown_builder::LabeledDrilldownBuilder;
    use extendable::Extendable;

    #[test]
    fn test_new() {
        let vanilla_select = SelectCommand::new("test".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_select);
    }

    #[test]
    fn test_filter() {
        let select = SelectCommand::new("test".to_string())
                         .filter("output_column @ \"type_safe\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("filter".to_string(),
                   "'output_column @ \"type_safe\"'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_query() {
        let select = SelectCommand::new("test".to_string())
                         .query("_key:\"http://example.org/\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("query".to_string(),
                   "\'_key:\"http://example.org/\"\'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_scorer() {
        let select = SelectCommand::new("test".to_string()).scorer("_score := rust".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("scorer".to_string(), "\'_score := rust\'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[allow(deprecated)]
    fn test_sortby() {
        let select = SelectCommand::new("test".to_string())
                         .sortby(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("sortby".to_string(), "\'test,piyo\'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_sort_keys() {
        let select = SelectCommand::new("test".to_string())
                         .sort_keys(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("sort_keys".to_string(), "\'test,piyo\'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_adjuster() {
        let select = SelectCommand::new("test".to_string())
                         .adjuster("content @ \"ruroonga\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("adjuster".to_string(),
                   "\'content @ \"ruroonga\"\'".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_match_columns() {
        let select = SelectCommand::new("test".to_string())
                         .match_columns(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("match_columns".to_string(), "test,piyo".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_output_columns() {
        let select = SelectCommand::new("test".to_string())
                         .output_columns(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("output_columns".to_string(), "test,piyo".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_offset() {
        let select = SelectCommand::new("test".to_string()).offset(100);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("offset".to_string(), "100".to_string());
        let expected = SelectCommand {
            command: Select,
            arguments: arg,
            table: "test".to_string(),
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_limit() {
        let select = SelectCommand::new("test".to_string()).limit(50);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("limit".to_string(), "50".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_cache() {
        let select_yes = SelectCommand::new("test".to_string()).cache(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("cache".to_string(), "yes".to_string());
        let expected_yes = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, select_yes);
        let select_no = SelectCommand::new("test".to_string()).cache(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("cache".to_string(), "no".to_string());
        let expected_no = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg_no,
        };
        assert_eq!(expected_no, select_no);
    }

    #[test]
    fn test_match_escalation_threshold() {
        let select = SelectCommand::new("test".to_string()).match_escalation_threshold(-1);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("match_escalation_threshold".to_string(), "-1".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_query_flags() {
        let select = SelectCommand::new("test".to_string())
                         .query_flags(vec![(QueryFlagsType::AllowColumn),
                                           (QueryFlagsType::AllowUpdate)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("query_flags".to_string(),
                   "ALLOW_COLUMN|ALLOW_UPDATE".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_query_expander() {
        let select = SelectCommand::new("test".to_string())
                         .query_expander(("Terms".to_string(), "synonym".to_string()));
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("query_expander".to_string(), "Terms.synonym".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "test".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_build() {
        let actual = SelectCommand::new("test".to_string())
                         .filter("output_column @ \"type_safe\"".to_string())
                         .build();
        let expected_query: Query = vec![("table".to_string(), "test".to_string()),
                                         ("filter".to_string(),
                                          "'output_column @ \"type_safe\"'".to_string())];
        let expected = (Select, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = SelectCommand::new("Test".to_string())
                        .filter("output_column @ \"type_safe\"".to_string())
                        .to_query();
        let url_encoded = "/d/select?table=Test&filter=%27output_column+%40+%22type_safe%22%27";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = SelectCommand::new("Test".to_string())
                        .filter("output_column @ \"type_safe\"".to_string())
                        .to_command();
        let cli_encoded = "select --table Test --filter \'output_column @ \"type_safe\"\'";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_add_ops_drilldown() {
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let ops_builder = (select.to_owned() + drilldown.to_owned()).build();
        let drilldown_builder = DrilldownBuilder::new(select.to_owned(), drilldown.to_owned())
                                    .build();
        assert_eq!(ops_builder, drilldown_builder);
    }

    #[test]
    fn test_add_ops_labedled_drilldown() {
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let drilldown = LabeledDrilldown::new("labeled".to_string())
                            .keys(vec![("tag".to_string())]);
        let ops_builder = (select.to_owned() + drilldown.to_owned()).build();
        let drilldown_builder = LabeledDrilldownBuilder::new(select.to_owned(),
                                                             drilldown.to_owned())
                                    .build();
        assert_eq!(ops_builder, drilldown_builder);
    }

    #[test]
    fn test_with_drilldown() {
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let drilldownable = select.to_owned().with_drilldown(drilldown.to_owned()).build();
        let drilldown_builder = DrilldownBuilder::new(select.to_owned(), drilldown.to_owned())
                                    .build();
        assert_eq!(drilldownable, drilldown_builder);
    }

    #[test]
    fn test_with_labeled_drilldown() {
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let drilldown = LabeledDrilldown::new("label".to_string()).keys(vec![("tag".to_string())]);
        let drilldownable = select.to_owned().with_labeled_drilldown(drilldown.to_owned()).build();
        let drilldown_builder = LabeledDrilldownBuilder::new(select.to_owned(),
                                                             drilldown.to_owned())
                                    .build();
        assert_eq!(drilldownable, drilldown_builder);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = SelectCommand {
            command: Select,
            table: "Test".to_string(),
            arguments: arg.to_owned(),
        };
        let query = SelectCommand::new("Test".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
