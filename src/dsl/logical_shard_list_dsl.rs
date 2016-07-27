use logical_shard_list::LogicalShardListCommand;

pub fn logical_shard_list(logical_table: String) -> LogicalShardListCommand {
    LogicalShardListCommand::new(logical_table)
}

#[cfg(test)]
mod test {
    use super::*;
    use logical_shard_list::LogicalShardListCommand;

    #[test]
    fn test_logical_shard_list() {
        let syntax = logical_shard_list("Logs".to_string());
        let actual = LogicalShardListCommand::new("Logs".to_string());
        assert_eq!(syntax, actual);
    }
}
