use io_flush::IoFlushCommand;

pub fn io_flush() -> IoFlushCommand {
    IoFlushCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use io_flush::IoFlushCommand;

    #[test]
    fn test_io_flush() {
        let syntax = io_flush();
        let actual = IoFlushCommand::new();
        assert_eq!(syntax, actual);
    }
}
