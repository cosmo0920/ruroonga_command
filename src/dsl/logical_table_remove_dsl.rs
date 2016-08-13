use logical_table_remove::LogicalTableRemoveCommand;

pub fn logical_table_remove(logical_table: String, shard_key: String) -> LogicalTableRemoveCommand {
    LogicalTableRemoveCommand::new(logical_table, shard_key)
}

#[cfg(test)]
mod test {
    use super::*;
    use logical_table_remove::LogicalTableRemoveCommand;

    #[test]
    fn test_logical_table_remove() {
        let syntax = logical_table_remove("Entries".to_string(), "created_at".to_string());
        let actual = LogicalTableRemoveCommand::new("Entries".to_string(), "created_at".to_string());
        assert_eq!(syntax, actual);
    }
}
