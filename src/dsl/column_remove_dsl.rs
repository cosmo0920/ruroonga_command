use column_remove::ColumnRemoveCommand;

pub fn column_remove(table: String, name: String) -> ColumnRemoveCommand {
    ColumnRemoveCommand::new(table, name)
}

#[cfg(test)]
mod test {
    use super::*;
    use column_remove::ColumnRemoveCommand;

    #[test]
    fn test_column_rename() {
        let syntax = column_remove("Table".to_string(), "column".to_string());
        let actual = ColumnRemoveCommand::new("Table".to_string(), "column".to_string());
        assert_eq!(syntax, actual);
    }
}
