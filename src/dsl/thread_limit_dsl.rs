use thread_limit::ThreadLimitCommand;

pub fn thread_limit() -> ThreadLimitCommand {
    ThreadLimitCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use thread_limit::ThreadLimitCommand;

    #[test]
    fn test_thread_limit() {
        let syntax = thread_limit();
        let actual = ThreadLimitCommand::new();
        assert_eq!(syntax, actual);
    }
}
