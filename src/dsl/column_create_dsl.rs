use column_create::ColumnCreateCommand;

pub fn column_create(table: String, name: String) -> ColumnCreateCommand {
    ColumnCreateCommand::new(table, name)
}

#[cfg(test)]
mod test {
    use super::*;
    use column_create::ColumnCreateCommand;

    #[test]
    fn test_column_create() {
        let syntax = column_create("Test".to_string(), "element".to_string());
        let actual = ColumnCreateCommand::new("Test".to_string(), "element".to_string());
        assert_eq!(syntax, actual);
    }
}
