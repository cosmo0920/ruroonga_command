use select::SelectCommand;
use command::Command;
#[cfg(feature="sharding")]
use logical_select::LogicalSelectCommand;
use selectable::fragmentable::OrderedFragment;

#[derive (Clone, PartialEq, Eq, Debug)]
pub struct StagedColumnsFragment {
    command: Command,
    query: OrderedFragment
}

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum DrilldownUsable {
    Select(SelectCommand),
    StagedColumns(StagedColumnsFragment),
    #[cfg(feature="sharding")]
    LogicalSelect(LogicalSelectCommand),
}
