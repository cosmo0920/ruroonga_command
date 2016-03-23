use dump::DumpCommand;

pub fn dump() -> DumpCommand {
    DumpCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use dump::DumpCommand;

    #[test]
    fn test_select() {
        let syntax = dump();
        let actual = DumpCommand::new();
        assert_eq!(syntax, actual);
    }
}
