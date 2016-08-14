use reindex::ReindexCommand;

pub fn reindex(target_name: String) -> ReindexCommand {
    ReindexCommand::new(target_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use reindex::ReindexCommand;

    #[test]
    fn test_reindex() {
        let syntax = reindex("Test".to_string());
        let actual = ReindexCommand::new("Test".to_string());
        assert_eq!(syntax, actual);
    }
}
