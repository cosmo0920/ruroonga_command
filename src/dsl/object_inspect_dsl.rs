use object_inspect::ObjectInspectCommand;

pub fn object_inspect() -> ObjectInspectCommand {
    ObjectInspectCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use object_inspect::ObjectInspectCommand;

    #[test]
    fn test_object_inspect() {
        let syntax = object_inspect();
        let actual = ObjectInspectCommand::new();
        assert_eq!(syntax, actual);
    }
}
