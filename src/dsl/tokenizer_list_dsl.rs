use super::super::tokenizer_list::TokenizerListCommand;

pub fn tokenizer_list() -> TokenizerListCommand {
    TokenizerListCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use tokenizer_list::TokenizerListCommand;

    #[test]
    fn test_tokenizer_list() {
        let syntax = tokenizer_list();
        let actual = TokenizerListCommand::new();
        assert_eq!(syntax, actual);
    }
}
