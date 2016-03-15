use super::super::delete::DeleteCommand;

pub fn delete(table: String) -> DeleteCommand {
    DeleteCommand::new(table)
}

#[cfg(test)]
mod test {
    use super::*;
    use delete::DeleteCommand;

    #[test]
    fn test_delete() {
        let syntax = delete("Test".to_string());
        let actual = DeleteCommand::new("Test".to_string());
        assert_eq!(syntax, actual);
    }
}
