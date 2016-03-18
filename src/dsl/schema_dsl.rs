use super::super::schema::SchemaCommand;

pub fn schema() -> SchemaCommand {
    SchemaCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use schema::SchemaCommand;

    #[test]
    fn test_schema() {
        let syntax = schema();
        let actual = SchemaCommand::new();
        assert_eq!(syntax, actual);
    }
}
