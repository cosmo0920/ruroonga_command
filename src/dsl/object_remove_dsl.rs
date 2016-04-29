use object_remove::ObjectRemoveCommand;

pub fn object_remove(name: String) -> ObjectRemoveCommand {
    ObjectRemoveCommand::new(name)
}

#[cfg(test)]
mod test {
    use super::*;
    use object_remove::ObjectRemoveCommand;

    #[test]
    fn test_object_remove() {
        let object = "object".to_string();
        let syntax = object_remove(object.clone());
        let actual = ObjectRemoveCommand::new(object.clone());
        assert_eq!(syntax, actual);
    }
}
