use select::SelectCommand;
use super::labeled_drilldown::LabeledDrilldown;
use command::{Command, Query};
use selectable::fragmentable::Fragmentable;
use command_query::CommandQuery;
use queryable::Queryable;
use commandable::Commandable;
use command_line::CommandLine;

pub struct LabeledDrilldownSequenceBuilder {
    select: SelectCommand,
    drilldowns: Vec<LabeledDrilldown>,
}

/// LabeledDrilldownSequence builder.
///
/// ## Simple usage
///
/// ```
/// use ruroonga_command::select::SelectCommand;
/// use ruroonga_command::selectable::labeled_drilldown::LabeledDrilldown;
/// use ruroonga_command::selectable::labeled_drilldown_sequence_builder as seq;
/// let select = SelectCommand::new("Memos".to_string());
/// let drilldown_label1 = LabeledDrilldown::new("label1".to_string())
///                  .table("label2".to_string())
///                  .keys(vec![("category".to_string())])
///                  .output_columns(vec![("_key".to_string()),
///                                       ("_nsubrecs".to_string())]);
/// let drilldown_label2 = LabeledDrilldown::new("label2".to_string())
///                  .keys(vec![("tag".to_string())])
///                  .output_columns(vec![("_key".to_string()),
///                                       ("_nsubrecs".to_string()),
///                                       ("category".to_string())]);
/// let mut builder =
///     seq::LabeledDrilldownSequenceBuilder::new(select,
///                                               vec![(drilldown_label1),
///                                                    (drilldown_label2)]);
/// let _ = builder.build();
/// ```
/// In more practical example, please refer to drilldown_builder examples
/// in [examples/drilldown_builder.rs]
/// (https://github.com/cosmo0920/ruroonga_command/blob/master/examples/advanced_drilldown.rs).

impl LabeledDrilldownSequenceBuilder {
    pub fn new(select: SelectCommand,
               drilldowns: Vec<LabeledDrilldown>)
               -> LabeledDrilldownSequenceBuilder {
        LabeledDrilldownSequenceBuilder {
            select: select,
            drilldowns: drilldowns,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        let (command, ordered_select, select) = self.select.to_fragment();
        for ordered in ordered_select.iter() {
            query.push(ordered.to_owned());
        }
        for (key, value) in select.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        for drilldown in self.drilldowns.iter() {
            let (_, _, drilldown) = drilldown.clone().to_fragment();
            for (key, value) in drilldown.iter() {
                query.push((key.to_owned(), value.to_owned()));
            }
        }
        (command, query)
    }
}

impl Queryable for LabeledDrilldownSequenceBuilder {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for LabeledDrilldownSequenceBuilder {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use queryable::Queryable;
    use commandable::Commandable;
    use select::SelectCommand;
    use selectable::labeled_drilldown::LabeledDrilldown;

    #[test]
    fn test_to_query() {
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let drilldown = LabeledDrilldown::new("label".to_string()).keys(vec![("tag".to_string())]);
        let builder = LabeledDrilldownSequenceBuilder::new(select, vec![(drilldown)]).to_query();
        let encoded = "/d/select?table=Entries&filter=%27content+%40+%22fast%22%27&drilldown%5Blab\
                       el%5D.keys=%27tag%27"
                          .to_string();
        assert_eq!(encoded, builder);
    }

    #[test]
    fn test_to_command() {
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let drilldown = LabeledDrilldown::new("label".to_string()).keys(vec![("tag".to_string())]);
        let builder = LabeledDrilldownSequenceBuilder::new(select, vec![(drilldown)]).to_command();
        let encoded = "select --table Entries --filter \'content @ \"fast\"\' \
                       --drilldown[label].keys \'tag\'"
                          .to_string();
        assert_eq!(encoded, builder);
    }
}