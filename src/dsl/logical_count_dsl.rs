use logical_count::LogicalCountCommand;

pub fn logical_count(logical_table: String, shard_key: String) -> LogicalCountCommand {
    LogicalCountCommand::new(logical_table, shard_key)
}

#[cfg(test)]
mod test {
    use super::*;
    use logical_count::LogicalCountCommand;

    #[test]
    fn test_logical_count() {
        let syntax = logical_count("Entries".to_string(), "created_at".to_string());
        let actual = LogicalCountCommand::new("Entries".to_string(),
                                              "created_at".to_string());
        assert_eq!(syntax, actual);
    }
}
