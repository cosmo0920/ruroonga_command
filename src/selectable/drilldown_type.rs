use select::SelectCommand;
#[cfg(feature="sharding")]
use logical_select::LogicalSelectCommand;

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum DrilldownUsable {
    Select(SelectCommand),
    #[cfg(feature="sharding")]
    LogicalSelect(LogicalSelectCommand),
}
