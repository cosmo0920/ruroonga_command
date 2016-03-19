use super::super::normalize::NormalizeCommand;
use types::normalizer_type::NormalizerType;

pub fn normalize(normalizer: NormalizerType, string: String) -> NormalizeCommand {
    NormalizeCommand::new(normalizer, string)
}

#[cfg(test)]
mod test {
    use super::*;
    use normalize::NormalizeCommand;
    use types::normalizer_type::NormalizerType;

    #[test]
    fn test_normalize() {
        let syntax = normalize(NormalizerType::Auto, "element".to_string());
        let actual = NormalizeCommand::new(NormalizerType::Auto, "element".to_string());
        assert_eq!(syntax, actual);
    }
}
