use selectable::labeled_drilldown::LabeledDrilldown;
use selectable::labeled_drilldown_sequence_builder::LabeledDrilldownSequenceBuilder;

pub trait LabeledDrilldownSequencable {
    fn with_labeled_drilldown_sequence(self,
                                       rhs: Vec<LabeledDrilldown>)
                                       -> LabeledDrilldownSequenceBuilder;
}
