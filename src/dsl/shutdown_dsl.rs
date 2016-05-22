use shutdown::ShutdownCommand;

pub fn shutdown() -> ShutdownCommand {
    ShutdownCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use shutdown::ShutdownCommand;

    #[test]
    fn test_shutdown() {
        let syntax = shutdown();
        let actual = ShutdownCommand::new();
        assert_eq!(syntax, actual);
    }
}
