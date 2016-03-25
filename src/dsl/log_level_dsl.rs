use log_level::LogLevelCommand;
use types::log_level_type::LogLevelType;

pub fn log_level(level: LogLevelType) -> LogLevelCommand {
    LogLevelCommand::new(level)
}

#[cfg(test)]
mod test {
    use super::*;
    use log_level::LogLevelCommand;
    use types::log_level_type::LogLevelType;

    #[test]
    fn test_log_level() {
        let syntax = log_level(LogLevelType::Warn);
        let actual = LogLevelCommand::new(LogLevelType::Warn);
        assert_eq!(syntax, actual);
    }
}
