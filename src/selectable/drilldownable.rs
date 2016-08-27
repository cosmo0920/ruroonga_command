use selectable::drilldown::Drilldown;
use selectable::drilldown_builder::DrilldownBuilder;

pub trait Drilldownable {
    fn with_drilldown(self, rhs: Drilldown) -> DrilldownBuilder;
}
