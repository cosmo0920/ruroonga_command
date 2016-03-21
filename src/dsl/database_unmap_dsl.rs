use super::super::database_unmap::DatabaseUnmapCommand;

pub fn database_unmap() -> DatabaseUnmapCommand {
    DatabaseUnmapCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use database_unmap::DatabaseUnmapCommand;

    #[test]
    fn test_database_unmap() {
        let syntax = database_unmap();
        let actual = DatabaseUnmapCommand::new();
        assert_eq!(syntax, actual);
    }
}
