use log_reopen::LogReopenCommand;

pub fn log_reopen() -> LogReopenCommand {
    LogReopenCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use log_reopen::LogReopenCommand;

    #[test]
    fn test_log_reopen() {
        let syntax = log_reopen();
        let actual = LogReopenCommand::new();
        assert_eq!(syntax, actual);
    }
}
