use table_remove::TableRemoveCommand;

pub fn table_remove(name: String) -> TableRemoveCommand {
    TableRemoveCommand::new(name)
}

#[cfg(test)]
mod test {
    use super::*;
    use table_remove::TableRemoveCommand;

    #[test]
    fn test_table_remove() {
        let syntax = table_remove("Table".to_string());
        let actual = TableRemoveCommand::new("Table".to_string());
        assert_eq!(syntax, actual);
    }
}
