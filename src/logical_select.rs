use super::command::{Command, Query};
use super::command::Command::LogicalSelect;
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
use selectable::drilldown_type::DrilldownUsable;
use types::range_filter_type::RangeFilterType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogicalSelectCommand {
    command: Command,
    logical_table: String,
    shard_key: String,
    arguments: HashMap<String, String>,
}

impl Default for LogicalSelectCommand {
    fn default() -> LogicalSelectCommand {
        LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "".to_string(),
            shard_key: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl LogicalSelectCommand {
    pub fn new(logical_table: String, shard_key: String) -> LogicalSelectCommand {
        let default: LogicalSelectCommand = Default::default();
        LogicalSelectCommand {
            command: default.command,
            logical_table: logical_table,
            shard_key: shard_key,
            arguments: default.arguments,
        }
    }

    pub fn filter(mut self, filter: String) -> LogicalSelectCommand {
        let encoded = format!("'{}'", filter);
        self.arguments.insert("filter".to_string(), encoded.to_owned());
        self
    }

    pub fn min(mut self, min: String) -> LogicalSelectCommand {
        let encoded = format!("'{}'", min);
        self.arguments.insert("min".to_string(), encoded.to_owned());
        self
    }

    pub fn min_border(mut self, border: RangeFilterType) -> LogicalSelectCommand {
        let encoded = format!("'{}'", border);
        self.arguments.insert("min_border".to_string(), encoded.to_owned());
        self
    }

    pub fn max(mut self, max: String) -> LogicalSelectCommand {
        let encoded = format!("'{}'", max);
        self.arguments.insert("max".to_string(), encoded.to_owned());
        self
    }

    pub fn max_border(mut self, border: RangeFilterType) -> LogicalSelectCommand {
        let encoded = format!("'{}'", border);
        self.arguments.insert("max_border".to_string(), encoded.to_owned());
        self
    }

    /// unimplemented!!
    #[allow(unused_mut, unused_variables)]
    pub fn query(mut self, query: String) -> LogicalSelectCommand {
        // let encoded = format!("'{}'", query);
        // self.arguments.insert("query".to_string(), encoded.to_owned());
        // self
        unimplemented!()
    }

    pub fn scorer(mut self, scorer: String) -> LogicalSelectCommand {
        let encoded = format!("'{}'", scorer);
        self.arguments.insert("scorer".to_string(), encoded.to_owned());
        self
    }

    #[deprecated]
    #[allow(deprecated)]
    pub fn sortby(mut self, targets: Vec<String>) -> LogicalSelectCommand {
        let string = util::split_values_vec(targets);
        let encoded = format!("'{}'", string);
        self.arguments.insert("sortby".to_string(), encoded.to_owned());
        self
    }

    pub fn sort_keys(mut self, targets: Vec<String>) -> LogicalSelectCommand {
        let string = util::split_values_vec(targets);
        let encoded = format!("'{}'", string);
        self.arguments.insert("sort_keys".to_string(), encoded.to_owned());
        self
    }

    /// unimplemented!!
    #[allow(unused_mut, unused_variables)]
    pub fn adjuster(mut self, adjust_expr: String) -> LogicalSelectCommand {
        // let encoded = format!("'{}'", adjust_expr);
        // self.arguments.insert("adjuster".to_string(), encoded.to_owned());
        // self
        unimplemented!()
    }

    /// unimplemented!!
    #[allow(unused_mut, unused_variables)]
    pub fn match_columns(mut self, columns: Vec<String>) -> LogicalSelectCommand {
        // let string = util::split_values_vec(columns);
        // self.arguments.insert("match_columns".to_string(), string.to_owned());
        // self
        unimplemented!()
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> LogicalSelectCommand {
        let string = util::split_values_vec(columns);
        self.arguments.insert("output_columns".to_string(), string.to_owned());
        self
    }

    pub fn offset(mut self, offset: i64) -> LogicalSelectCommand {
        let offset = format!("{}", offset);
        self.arguments.insert("offset".to_string(), offset.to_owned());
        self
    }

    pub fn limit(mut self, limit: i64) -> LogicalSelectCommand {
        let limit = format!("{}", limit);
        self.arguments.insert("limit".to_string(), limit.to_owned());
        self
    }

    /// unimplemented!!
    #[allow(unused_mut, unused_variables)]
    pub fn cache(mut self, cache: bool) -> LogicalSelectCommand {
        // let flag = if cache {
        //     "yes"
        // } else {
        //     "no"
        // };
        // self.arguments.insert("cache".to_string(), flag.to_string());
        // self
        unimplemented!()
    }

    /// unimplemented!!
    #[allow(unused_mut, unused_variables)]
    pub fn match_escalation_threshold(mut self, threshold: i64) -> LogicalSelectCommand {
        // let string = format!("{}", threshold);
        // self.arguments.insert("match_escalation_threshold".to_string(), string.to_owned());
        // self
        unimplemented!()
    }

    /// unimplemented!!
    #[allow(unused_mut, unused_variables)]
    pub fn query_flags(mut self, flags: Vec<QueryFlagsType>) -> LogicalSelectCommand {
        // let string = util::split_flags_vec(flags);
        // self.arguments.insert("query_flags".to_string(), string.to_owned());
        // self
        unimplemented!()
    }

    /// unimplemented!!
    #[allow(unused_mut, unused_variables)]
    pub fn query_expander(mut self, synonym: (String, String)) -> LogicalSelectCommand {
        // let (table, column) = synonym;
        // let string = format!("{}.{}", table, column);
        // self.arguments.insert("query_expander".to_string(), string.to_owned());
        // self
        unimplemented!()
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![("logical_table".to_string(), self.logical_table),
                                    ("shard_key".to_string(), self.shard_key)];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        (LogicalSelect, query)
    }
}

impl Queryable for LogicalSelectCommand {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LogicalSelectCommand {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

impl Fragmentable for LogicalSelectCommand {
    fn to_fragment(self) -> (Command, OrderedFragment, QueryFragment) {
        let mut select_fragment = HashMap::new();
        let ordered_fragment = vec![("logical_table".to_string(), self.logical_table),
                                    ("shard_key".to_string(), self.shard_key)];
        for (key, value) in &self.arguments {
            select_fragment.insert(key.to_owned(), value.to_owned());
        }
        (LogicalSelect, ordered_fragment, select_fragment)
    }
}

impl Add<Drilldown> for LogicalSelectCommand {
    type Output = DrilldownBuilder;

    fn add(self, rhs: Drilldown) -> DrilldownBuilder {
        DrilldownBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}

impl Add<LabeledDrilldown> for LogicalSelectCommand {
    type Output = LabeledDrilldownBuilder;

    fn add(self, rhs: LabeledDrilldown) -> LabeledDrilldownBuilder {
        LabeledDrilldownBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}

impl Add<Vec<LabeledDrilldown>> for LogicalSelectCommand {
    type Output = LabeledDrilldownSequenceBuilder;

    fn add(self, rhs: Vec<LabeledDrilldown>) -> LabeledDrilldownSequenceBuilder {
        LabeledDrilldownSequenceBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}

impl Add<StagedColumns> for LogicalSelectCommand {
    type Output = StagedColumnsBuilder;

    fn add(self, rhs: StagedColumns) -> StagedColumnsBuilder {
        StagedColumnsBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}

impl Drilldownable for LogicalSelectCommand {
    fn with_drilldown(self, rhs: Drilldown) -> DrilldownBuilder {
        DrilldownBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}

impl LabeledDrilldownable for LogicalSelectCommand {
    fn with_labeled_drilldown(self, rhs: LabeledDrilldown) -> LabeledDrilldownBuilder {
        LabeledDrilldownBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}


impl LabeledDrilldownSequencable for LogicalSelectCommand {
    fn with_labeled_drilldown_sequence(self,
                                       rhs: Vec<LabeledDrilldown>)
                                       -> LabeledDrilldownSequenceBuilder {
        LabeledDrilldownSequenceBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}

impl StagedColumnable for LogicalSelectCommand {
    fn with_staged_columns(self, rhs: StagedColumns) -> StagedColumnsBuilder {
        StagedColumnsBuilder::new(DrilldownUsable::LogicalSelect(self), rhs)
    }
}

extendable!(LogicalSelectCommand);
request_cancellable!(LogicalSelectCommand);
request_timeoutable!(LogicalSelectCommand);

#[cfg(feature="sharding")]
#[cfg(test)]
mod test {
    use super::*;
    use command::Query;
    use command::Command::LogicalSelect;
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
    use selectable::drilldown_type::DrilldownUsable;
    use types::range_filter_type::RangeFilterType::{Include, Exclude};

    #[test]
    fn test_new() {
        let vanilla_select = LogicalSelectCommand::new("Entries".to_string(),
                                                       "created_at".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_select);
    }

    #[test]
    fn test_filter() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("output_column @ \"type_safe\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("filter".to_string(),
                   "'output_column @ \"type_safe\"'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_min() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .min("2016/07/23 00:00:00".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("min".to_string(), "'2016/07/23 00:00:00'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_min_border() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .min("2016/07/23 00:00:00".to_string())
            .min_border(Exclude);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("min".to_string(), "'2016/07/23 00:00:00'".to_string());
        arg.insert("min_border".to_string(), "'exclude'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_max() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .max("2016/07/23 00:00:00".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "'2016/07/23 00:00:00'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_max_border() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .max("2016/07/23 00:00:00".to_string())
            .max_border(Include);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("max".to_string(), "'2016/07/23 00:00:00'".to_string());
        arg.insert("max_border".to_string(), "'include'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[should_panic]
    fn test_query() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .query("_key:\"http://example.org/\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("query".to_string(),
                   "\'_key:\"http://example.org/\"\'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_scorer() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .scorer("_score := rust".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("scorer".to_string(), "\'_score := rust\'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[allow(deprecated)]
    fn test_sortby() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .scorer("_score := rust".to_string())
            .sortby(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("sortby".to_string(), "\'test,piyo\'".to_string());
        arg.insert("scorer".to_string(), "\'_score := rust\'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_sort_keys() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .scorer("_score := rust".to_string())
            .sort_keys(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("sort_keys".to_string(), "\'test,piyo\'".to_string());
        arg.insert("scorer".to_string(), "\'_score := rust\'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[should_panic]
    fn test_adjuster() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .adjuster("content @ \"ruroonga\"".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("adjuster".to_string(),
                   "\'content @ \"ruroonga\"\'".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[should_panic]
    fn test_match_columns() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .match_columns(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("match_columns".to_string(), "test,piyo".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_output_columns() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .output_columns(vec!["test".to_string(), "piyo".to_string()]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("output_columns".to_string(), "test,piyo".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_offset() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .offset(100);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("offset".to_string(), "100".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_limit() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .limit(50);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("limit".to_string(), "50".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[should_panic]
    fn test_cache() {
        let select_yes = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .cache(true);
        let mut arg_yes: HashMap<String, String> = HashMap::new();
        arg_yes.insert("cache".to_string(), "yes".to_string());
        let expected_yes = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg_yes,
        };
        assert_eq!(expected_yes, select_yes);
        let select_no = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .cache(false);
        let mut arg_no: HashMap<String, String> = HashMap::new();
        arg_no.insert("cache".to_string(), "no".to_string());
        let expected_no = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg_no,
        };
        assert_eq!(expected_no, select_no);
    }

    #[test]
    #[should_panic]
    fn test_match_escalation_threshold() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .match_escalation_threshold(-1);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("match_escalation_threshold".to_string(), "-1".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[should_panic]
    fn test_query_flags() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .query_flags(vec![(QueryFlagsType::AllowColumn), (QueryFlagsType::AllowUpdate)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("query_flags".to_string(),
                   "ALLOW_COLUMN|ALLOW_UPDATE".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    #[should_panic]
    fn test_query_expander() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .query_expander(("Terms".to_string(), "synonym".to_string()));
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("query_expander".to_string(), "Terms.synonym".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg,
        };
        assert_eq!(expected, select);
    }

    #[test]
    fn test_build() {
        let actual = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("output_column @ \"type_safe\"".to_string())
            .build();
        let expected_query: Query = vec![("logical_table".to_string(), "Entries".to_string()),
                                         ("shard_key".to_string(), "created_at".to_string()),
                                         ("filter".to_string(),
                                          "'output_column @ \"type_safe\"'".to_string())];
        let expected = (LogicalSelect, expected_query);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_queryable() {
        let query = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("output_column @ \"type_safe\"".to_string())
            .to_query();
        let url_encoded = "/d/logical_select?logical_table=Entries&shard_key=\
                           created_at&filter=%27output_column+%40+%22type_safe%22%27";
        assert_eq!(url_encoded.to_string(), query);
    }

    #[test]
    fn test_commandable() {
        let query = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("output_column @ \"type_safe\"".to_string())
            .to_command();
        let cli_encoded = "logical_select --logical_table Entries --shard_key created_at --filter \
                           \'output_column @ \"type_safe\"\'";
        assert_eq!(cli_encoded.to_string(), query);
    }

    #[test]
    fn test_add_ops_drilldown() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("content @ \"fast\"".to_string());
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let ops_builder = (select.to_owned() + drilldown.to_owned()).build();
        let drilldown_builder =
            DrilldownBuilder::new(DrilldownUsable::LogicalSelect(select.to_owned()),
                                  drilldown.to_owned())
                .build();
        assert_eq!(ops_builder, drilldown_builder);
    }

    #[test]
    fn test_add_ops_labedled_drilldown() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("content @ \"fast\"".to_string());
        let drilldown = LabeledDrilldown::new("labeled".to_string())
            .keys(vec![("tag".to_string())]);
        let ops_builder = (select.to_owned() + drilldown.to_owned()).build();
        let drilldown_builder =
            LabeledDrilldownBuilder::new(DrilldownUsable::LogicalSelect(select.to_owned()),
                                         drilldown.to_owned())
                .build();
        assert_eq!(ops_builder, drilldown_builder);
    }

    #[test]
    fn test_with_drilldown() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("content @ \"fast\"".to_string());
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let drilldownable = select.to_owned().with_drilldown(drilldown.to_owned()).build();
        let drilldown_builder =
            DrilldownBuilder::new(DrilldownUsable::LogicalSelect(select.to_owned()),
                                  drilldown.to_owned())
                .build();
        assert_eq!(drilldownable, drilldown_builder);
    }

    #[test]
    fn test_with_labeled_drilldown() {
        let select = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string())
            .filter("content @ \"fast\"".to_string());
        let drilldown = LabeledDrilldown::new("label".to_string()).keys(vec![("tag".to_string())]);
        let drilldownable = select.to_owned().with_labeled_drilldown(drilldown.to_owned()).build();
        let drilldown_builder =
            LabeledDrilldownBuilder::new(DrilldownUsable::LogicalSelect(select.to_owned()),
                                         drilldown.to_owned())
                .build();
        assert_eq!(drilldownable, drilldown_builder);
    }

    #[test]
    fn test_extendable() {
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("user".to_string(), "defined".to_string());
        let expected = LogicalSelectCommand {
            command: LogicalSelect,
            logical_table: "Entries".to_string(),
            shard_key: "created_at".to_string(),
            arguments: arg.to_owned(),
        };
        let query = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string());
        unsafe {
            let extended = query.set_arguments(arg.to_owned());
            assert_eq!(expected, extended);
        }
    }
}
