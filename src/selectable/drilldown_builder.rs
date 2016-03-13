use select::SelectCommand;
use super::drilldown::Drilldown;
use command::{Command, Query};
use command::Command::Select;
use selectable::fragmentable::Fragmentable;

pub struct DrilldownBuilder {
    select: SelectCommand,
    drilldown: Drilldown,
}

impl DrilldownBuilder {
    pub fn new(select: SelectCommand, drilldown: Drilldown) -> DrilldownBuilder {
        DrilldownBuilder {
            select: select,
            drilldown: drilldown,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        let select = self.select.to_fragment();
        for (key, value) in select.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        let drilldown = self.drilldown.to_fragment();
        for (key, value) in drilldown.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        (Select, query)
    }
}
