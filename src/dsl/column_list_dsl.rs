use column_list::ColumnListCommand;

pub fn column_list() -> ColumnListCommand {
    ColumnListCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use column_list::ColumnListCommand;

    #[test]
    fn test_column_list() {
        let syntax = column_list();
        let actual = ColumnListCommand::new();
        assert_eq!(syntax, actual);
    }
}
