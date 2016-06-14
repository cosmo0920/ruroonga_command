use selectable::staged_columns::StagedColumns;
use selectable::staged_columns_builder::StagedColumnsBuilder;

pub trait StagedColumnable {
    fn with_staged_columns(self, rhs: StagedColumns) -> StagedColumnsBuilder;
}
