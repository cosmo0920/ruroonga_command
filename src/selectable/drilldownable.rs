use super::drilldown::Drilldown;
use super::drilldown_builder::DrilldownBuilder;

pub trait Drilldownable {
    fn with_drilldown(self, rhs: Drilldown) -> DrilldownBuilder;
}
