use super::super::request_cancel::RequestCancelCommand;

pub fn request_cancel(id: String) -> RequestCancelCommand {
    RequestCancelCommand::new(id)
}

#[cfg(test)]
mod test {
    use super::*;
    use request_cancel::RequestCancelCommand;

    #[test]
    fn test_request_cancel() {
        let syntax = request_cancel("cancel-id".to_string());
        let actual = RequestCancelCommand::new("cancel-id".to_string());
        assert_eq!(syntax, actual);
    }
}
