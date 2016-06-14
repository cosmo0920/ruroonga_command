use select::SelectCommand;
use selectable::staged_columns::StagedColumns;
use command::{Command, Query};
use selectable::fragmentable::Fragmentable;
use command_query::CommandQuery;
use queryable::Queryable;
use commandable::Commandable;
use command_line::CommandLine;

pub struct StagedColumnsBuilder {
    select: SelectCommand,
    staged_columns: StagedColumns,
}

/// `StagedColumnsBuilder` builder.
///
/// ## Simple usage
///
/// ```
/// use ruroonga_command::select::SelectCommand;
/// use ruroonga_command::selectable::staged_columns::StagedColumns;
/// use ruroonga_command::selectable::staged_columns_builder::StagedColumnsBuilder;
/// use ruroonga_command::types::data_type::DataType;
/// use ruroonga_command::types::column_flag_type::ColumnFlagType;
/// let label = "label1".to_string();
/// let stage = "staged".to_string();
/// let value = "'_id'".to_string();
/// let select = SelectCommand::new("Entries".to_string())
///                  .filter("content @ \"fast\"".to_string());
/// let staged_columns = StagedColumns::new(label.clone(),
///                                         stage.clone(),
///                                         DataType::UInt32,
///                                         vec![(ColumnFlagType::Scalar)],
///                                         value.clone());
/// let builder = StagedColumnsBuilder::new(select, staged_columns);
/// let _ = builder.build();
/// ```

impl StagedColumnsBuilder {
    pub fn new(select: SelectCommand, staged_columns: StagedColumns) -> StagedColumnsBuilder {
        StagedColumnsBuilder {
            select: select,
            staged_columns: staged_columns,
        }
    }

    pub fn build(self) -> (Command, Query) {
        let mut query: Query = vec![];
        let (command, ordered_select, select) = self.select.to_fragment();
        for ordered in &ordered_select {
            query.push(ordered.to_owned());
        }
        for (key, value) in &select {
            query.push((key.to_owned(), value.to_owned()));
        }
        let (_, _, staged_columns) = self.staged_columns.construct().to_fragment();
        for (key, value) in &staged_columns {
            query.push((key.to_owned(), value.to_owned()));
        }
        (command, query)
    }
}

impl Queryable for StagedColumnsBuilder {
    fn to_query(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandQuery::new(command, query);
        command.encode()
    }
}

impl Commandable for StagedColumnsBuilder {
    fn to_command(self) -> String {
        let (command, query) = self.build();
        let mut command = CommandLine::new(command, query);
        command.encode()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use queryable::Queryable;
    use commandable::Commandable;
    use select::SelectCommand;
    use types::data_type::DataType;
    use types::column_flag_type::ColumnFlagType;
    use selectable::staged_columns::StagedColumns;

    #[test]
    #[ignore]
    // Too complex test
    fn test_to_query() {
        let label = "label1".to_string();
        let stage = "staged".to_string();
        let value = "'_id'".to_string();
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let staged_columns = StagedColumns::new(label.clone(),
                                                stage.clone(),
                                                DataType::UInt32,
                                                vec![(ColumnFlagType::Scalar)],
                                                value.clone());
        let builder = StagedColumnsBuilder::new(select, staged_columns).to_query();
        let encoded = "/d/select?table=Entries&filter=%27content+%40+%22fast%22%27&columns%5Blabel\
                       1%5D.value=%27_id%27&columns%5Blabel1%5D.type=UInt32&columns%5Blabel1%5D.\
                       flags=%27COLUMN_SCALAR%27&columns%5Blabel1%5D.stage=staged"
                          .to_string();
        assert_eq!(encoded, builder);
    }

    #[test]
    #[ignore]
    // Too complex test
    fn test_to_command() {
        let label = "label1".to_string();
        let stage = "staged".to_string();
        let value = "'_id'".to_string();
        let select = SelectCommand::new("Entries".to_string())
                         .filter("content @ \"fast\"".to_string());
        let staged_columns = StagedColumns::new(label.clone(),
                                                stage.clone(),
                                                DataType::UInt32,
                                                vec![(ColumnFlagType::Scalar)],
                                                value.clone());
        let builder = StagedColumnsBuilder::new(select, staged_columns).to_command();
        let encoded = "select --table Entries --filter \'content @ \"fast\"\' \
                       --columns[label1].value \'_id\' --columns[label1].stage staged \
                       --columns[label1].flags \'COLUMN_SCALAR\' --columns[label1].type UInt32"
                          .to_string();
        assert_eq!(encoded, builder);
    }
}
