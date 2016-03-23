use log_put::LogPutCommand;
use types::log_level_type::LogLevelType;

pub fn log_put(level: LogLevelType, message: String) -> LogPutCommand {
    LogPutCommand::new(level, message)
}

#[cfg(test)]
mod test {
    use super::*;
    use log_put::LogPutCommand;
    use types::log_level_type::LogLevelType;

    #[test]
    fn test_log_put() {
        let syntax = log_put(LogLevelType::Warn, "test".to_string());
        let actual = LogPutCommand::new(LogLevelType::Warn, "test".to_string());
        assert_eq!(syntax, actual);
    }
}
