use super::super::table_create::TableCreateCommand;

pub fn table_create(name: String) -> TableCreateCommand {
    TableCreateCommand::new(name)
}

#[cfg(test)]
mod test {
    use super::*;
    use table_create::TableCreateCommand;

    #[test]
    fn test_table_create() {
        let syntax = table_create("Test".to_string());
        let actual = TableCreateCommand::new("Test".to_string());
        assert_eq!(syntax, actual);
    }
}
