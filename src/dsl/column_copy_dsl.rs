use column_copy::ColumnCopyCommand;

pub fn column_copy(from_table: String,
                   from_name: String,
                   to_table: String,
                   to_name: String)
                   -> ColumnCopyCommand {
    ColumnCopyCommand::new(from_table, from_name, to_table, to_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use column_copy::ColumnCopyCommand;

    #[test]
    fn test_column_copy() {
        let syntax = column_copy("Test".to_string(),
                                 "element".to_string(),
                                 "NewTest".to_string(),
                                 "new_element".to_string());
        let actual = ColumnCopyCommand::new("Test".to_string(),
                                            "element".to_string(),
                                            "NewTest".to_string(),
                                            "new_element".to_string());
        assert_eq!(syntax, actual);
    }
}
