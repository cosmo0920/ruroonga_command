use logical_select::LogicalSelectCommand;

pub fn logical_select(logical_table: String, shard_key: String) -> LogicalSelectCommand {
    LogicalSelectCommand::new(logical_table, shard_key)
}

#[cfg(test)]
mod test {
    use super::*;
    use logical_select::LogicalSelectCommand;

    #[test]
    fn test_logical_select() {
        let syntax = logical_select("Entries".to_string(), "created_at".to_string());
        let actual = LogicalSelectCommand::new("Entries".to_string(), "created_at".to_string());
        assert_eq!(syntax, actual);
    }
}
