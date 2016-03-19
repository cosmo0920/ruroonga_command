use super::super::tokenize::TokenizeCommand;
use types::tokenizer_type::TokenizerType;

pub fn tokenize(tokenizer: TokenizerType, string: String) -> TokenizeCommand {
    TokenizeCommand::new(tokenizer, string)
}

#[cfg(test)]
mod test {
    use super::*;
    use tokenize::TokenizeCommand;
    use types::tokenizer_type::TokenizerType;

    #[test]
    fn test_tokenize() {
        let syntax = tokenize(TokenizerType::Bigram, "element".to_string());
        let actual = TokenizeCommand::new(TokenizerType::Bigram, "element".to_string());
        assert_eq!(syntax, actual);
    }
}
