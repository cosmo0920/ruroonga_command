use cache_limit::CacheLimitCommand;

pub fn cache_limit() -> CacheLimitCommand {
    CacheLimitCommand::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use cache_limit::CacheLimitCommand;

    #[test]
    fn test_cache_limit() {
        let syntax = cache_limit();
        let actual = CacheLimitCommand::new();
        assert_eq!(syntax, actual);
    }
}
