use select::SelectCommand;
use super::drilldown::Drilldown;
use command::{Command, Query};
use command::Command::Select;
use selectable::fragmentable::Fragmentable;
use command_query::CommandQuery;
use queryable::Queryable;
use commandable::Commandable;
use command_line::CommandLine;

pub struct DrilldownBuilder {
    select: SelectCommand,
    drilldown: Drilldown,
}

/// Drilldown builder.
///
/// ## Simple usage
///
/// ```
/// extern crate ruroonga_command as ruroonga;
/// let select = ruroonga::select::SelectCommand::new("Entries".to_string())
///                  .filter("content @ \"fast\"".to_string());
/// let drilldown = ruroonga::selectable::drilldown::Drilldown::new()
///                  .drilldown("tag".to_string());
/// let mut builder =
///     ruroonga::selectable::drilldown_builder::DrilldownBuilder::new(select, drilldown);
/// let _ = builder.build();
/// ```
/// In more practical example, please refer to drilldown_builder_xxx
/// in [examples/drilldown_builder.rs]
/// (https://github.com/cosmo0920/ruroonga_command/blob/master/examples/drilldown_builder.rs).
impl DrilldownBuilder {
    pub fn new(select: SelectCommand, drilldown: Drilldown) -> DrilldownBuilder {
        DrilldownBuilder {
            select: select,
            drilldown: drilldown,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        let (ordered_select, select) = self.select.to_fragment();
        for ordered in ordered_select.iter() {
            query.push(ordered.to_owned());
        }
        for (key, value) in select.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        let (_, drilldown) = self.drilldown.to_fragment();
        for (key, value) in drilldown.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Select, query)
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
