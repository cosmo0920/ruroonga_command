use super::super::table_rename::TableRenameCommand;

pub fn table_rename(name: String, new_name: String) -> TableRenameCommand {
    TableRenameCommand::new(name, new_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use table_rename::TableRenameCommand;

    #[test]
    fn test_table_rename() {
        let syntax = table_rename("old".to_string(), "new".to_string());
        let actual = TableRenameCommand::new("old".to_string(), "new".to_string());
        assert_eq!(syntax, actual);
    }
}
