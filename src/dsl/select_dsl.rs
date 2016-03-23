use select::SelectCommand;

pub fn select(table: String) -> SelectCommand {
    SelectCommand::new(table)
}

#[cfg(test)]
mod test {
    use super::*;
    use select::SelectCommand;

    #[test]
    fn test_select() {
        let syntax = select("Test".to_string());
        let actual = SelectCommand::new("Test".to_string());
        assert_eq!(syntax, actual);
    }
}
