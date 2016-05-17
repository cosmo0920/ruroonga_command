use object_exist::ObjectExistCommand;

pub fn object_exist(name: String) -> ObjectExistCommand {
    ObjectExistCommand::new(name)
}

#[cfg(test)]
mod test {
    use super::*;
    use object_exist::ObjectExistCommand;

    #[test]
    fn test_object_exist() {
        let object = "object".to_string();
        let syntax = object_exist(object.to_owned());
        let actual = ObjectExistCommand::new(object.to_owned());
        assert_eq!(syntax, actual);
    }
}
