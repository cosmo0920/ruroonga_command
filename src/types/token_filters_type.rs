use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::TokenFiltersType::{StopWord, Stem, ExtTokenFiltersType};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum TokenFiltersType {
    StopWord,
    Stem,
    /// For future extensibility.
    ExtTokenFiltersType(String),
}

impl AsRef<str> for TokenFiltersType {
    fn as_ref(&self) -> &str {
        match *self {
            StopWord => "TokenFilterStopWord",
            Stem => "TokenFilterStem",
            ExtTokenFiltersType(ref s) => s.as_ref()
        }
    }
}

#[derive (Debug)]
pub enum TokenFiltersTypeError { Empty }

impl FromStr for TokenFiltersType {
    type Err = TokenFiltersTypeError;
    fn from_str(s: &str) -> Result<TokenFiltersType, TokenFiltersTypeError> {
        if s == "" {
            Err(TokenFiltersTypeError::Empty)
        } else {
            Ok(match s {
                "StopWord" | "TokenFilterStopWord" => StopWord,
                "Stem" | "TokenFilterStem" => Stem,
                _ => ExtTokenFiltersType(s.to_owned())
            })
        }
    }
}

impl fmt::Display for TokenFiltersType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            StopWord => "TokenFilterStopWord",
            Stem => "TokenFilterStem",
            ExtTokenFiltersType(ref s) => s.as_ref()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(TokenFiltersType::StopWord, FromStr::from_str("StopWord").unwrap());
        assert_eq!(TokenFiltersType::StopWord, FromStr::from_str("TokenFilterStopWord").unwrap());
        assert_eq!(TokenFiltersType::ExtTokenFiltersType("AddedFilter".to_owned()),
                   FromStr::from_str("AddedFilter").unwrap());
        let x: Result<TokenFiltersType, _> = FromStr::from_str("");
        if let Err(TokenFiltersTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("TokenFilterStem".to_owned(), format!("{}", TokenFiltersType::Stem));
        assert_eq!("AddedFilter".to_owned(),
                   format!("{}", TokenFiltersType::ExtTokenFiltersType("AddedFilter".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(TokenFiltersType::Stem.as_ref(), "TokenFilterStem");
        assert_eq!(TokenFiltersType::ExtTokenFiltersType("AddedFilter".to_owned()).as_ref(),
                   "AddedFilter");
    }
}
