use column_rename::ColumnRenameCommand;

pub fn column_rename(name: String, new_name: String) -> ColumnRenameCommand {
    ColumnRenameCommand::new(name, new_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use column_rename::ColumnRenameCommand;

    #[test]
    fn test_column_rename() {
        let syntax = column_rename("old".to_string(), "new".to_string());
        let actual = ColumnRenameCommand::new("old".to_string(), "new".to_string());
        assert_eq!(syntax, actual);
    }
}
