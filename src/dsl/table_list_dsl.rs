use table_list::TableListCommand;

pub fn table_list() -> TableListCommand {
    TableListCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use table_list::TableListCommand;

    #[test]
    fn test_table_list() {
        let syntax = table_list();
        let actual = TableListCommand::new();
        assert_eq!(syntax, actual);
    }
}
