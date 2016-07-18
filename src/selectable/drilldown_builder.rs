use super::drilldown::Drilldown;
use command::{Command, Query};
use selectable::fragmentable::Fragmentable;
use command_query::CommandQuery;
use queryable::Queryable;
use commandable::Commandable;
use command_line::CommandLine;
use selectable::drilldown_type::DrilldownUsable;

pub struct DrilldownBuilder {
    select: DrilldownUsable,
    drilldown: Drilldown,
}

/// Drilldown builder.
///
/// ## Simple usage
///
/// ```
/// use ruroonga_command::select::SelectCommand;
/// use ruroonga_command::selectable::drilldown::Drilldown;
/// use ruroonga_command::selectable::drilldown_builder::DrilldownBuilder;
/// use ruroonga_command::selectable::drilldown_type::DrilldownUsable;
/// let select = SelectCommand::new("Entries".to_string())
///                  .filter("content @ \"fast\"".to_string());
/// let drilldown = Drilldown::new()
///                  .drilldown(vec![("tag".to_string())]);
/// let mut builder = DrilldownBuilder::new(DrilldownUsable::Select(select), drilldown);
/// let _ = builder.build();
/// ```
/// In more practical example, please refer to `drilldown_builder` examples
/// in [examples/drilldown_builder.rs]
/// (https://github.com/cosmo0920/ruroonga_command/blob/master/examples/drilldown_builder.rs).
impl DrilldownBuilder {
    pub fn new(select: DrilldownUsable, drilldown: Drilldown) -> DrilldownBuilder {
        DrilldownBuilder {
            select: select,
            drilldown: drilldown,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        let (command, ordered_select, key_values) = match self.select {
            DrilldownUsable::Select(s) => s.to_fragment(),
            #[cfg(feature="sharding")]
            DrilldownUsable::LogicalSelect(l) => l.to_fragment(),
        };
        for ordered in &ordered_select {
            query.push(ordered.to_owned());
        }
        for (key, value) in &key_values {
            query.push((key.to_owned(), value.to_owned()));
        }
        let (_, _, drilldown) = self.drilldown.to_fragment();
        for (key, value) in &drilldown {
            query.push((key.to_owned(), value.to_owned()));
        }
        (command, query)
    }
}

impl Queryable for DrilldownBuilder {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for DrilldownBuilder {
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
    use selectable::drilldown::Drilldown;
    use selectable::drilldown_type::DrilldownUsable;

    #[test]
    fn test_to_query() {
        let select = SelectCommand::new("Entries".to_string())
            .filter("content @ \"fast\"".to_string());
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let builder = DrilldownBuilder::new(DrilldownUsable::Select(select), drilldown).to_query();
        let encoded = "/d/select?table=Entries&filter=%27content+%40+%22fast%22%27&drilldown=%27ta\
                       g%27"
            .to_string();
        assert_eq!(encoded, builder);
    }

    #[test]
    fn test_to_command() {
        let select = SelectCommand::new("Entries".to_string())
            .filter("content @ \"fast\"".to_string());
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let builder = DrilldownBuilder::new(DrilldownUsable::Select(select), drilldown).to_command();
        let encoded = "select --table Entries --filter \'content @ \"fast\"\' --drilldown \'tag\'"
            .to_string();
        assert_eq!(encoded, builder);
    }
}
