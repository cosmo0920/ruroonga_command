use truncate::TruncateCommand;

pub fn truncate(target_name: String) -> TruncateCommand {
    TruncateCommand::new(target_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use truncate::TruncateCommand;

    #[test]
    fn test_truncate() {
        let syntax = truncate("Test".to_string());
        let actual = TruncateCommand::new("Test".to_string());
        assert_eq!(syntax, actual);
    }
}
