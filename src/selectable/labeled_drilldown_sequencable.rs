use super::labeled_drilldown::LabeledDrilldown;
use super::labeled_drilldown_sequence_builder::LabeledDrilldownSequenceBuilder;

pub trait LabeledDrilldownSequencable {
    fn with_labeled_drilldown_sequence(self, rhs: Vec<LabeledDrilldown>) -> LabeledDrilldownSequenceBuilder;
}
