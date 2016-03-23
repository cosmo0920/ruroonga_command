use load::LoadCommand;

pub fn load(table: String, data: String) -> LoadCommand {
    LoadCommand::new(table, data)
}

#[cfg(test)]
mod test {
    use super::*;
    use load::LoadCommand;

    #[test]
    fn test_load() {
        let syntax = load("Test".to_string(), "[]".to_string());
        let actual = LoadCommand::new("Test".to_string(), "[]".to_string());
        assert_eq!(syntax, actual);
    }
}
