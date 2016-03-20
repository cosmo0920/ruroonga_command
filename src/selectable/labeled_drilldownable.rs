use super::labeled_drilldown::LabeledDrilldown;
use super::labeled_drilldown_builder::LabeledDrilldownBuilder;

pub trait LabeledDrilldownable {
    fn with_labeled_drilldown(self, rhs: LabeledDrilldown) -> LabeledDrilldownBuilder;
}
