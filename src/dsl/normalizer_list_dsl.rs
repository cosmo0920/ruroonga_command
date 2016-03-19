use super::super::normalizer_list::NormalizerListCommand;

pub fn normalizer_list() -> NormalizerListCommand {
    NormalizerListCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use normalizer_list::NormalizerListCommand;

    #[test]
    fn test_normalizer_list() {
        let syntax = normalizer_list();
        let actual = NormalizerListCommand::new();
        assert_eq!(syntax, actual);
    }
}
