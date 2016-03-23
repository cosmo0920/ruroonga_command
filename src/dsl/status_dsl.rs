use status::StatusCommand;

pub fn status() -> StatusCommand {
    StatusCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use status::StatusCommand;

    #[test]
    fn test_status() {
        let syntax = status();
        let actual = StatusCommand::new();
        assert_eq!(syntax, actual);
    }
}
