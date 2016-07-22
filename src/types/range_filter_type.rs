use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::RangeFilterType::{Include, Exclude, ExtRangeFilterType};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum RangeFilterType {
    Include,
    Exclude,
    /// For future extensibility.
    ExtRangeFilterType(String),
}

impl AsRef<str> for RangeFilterType {
    fn as_ref(&self) -> &str {
        match *self {
            Include => "include",
            Exclude => "exclude",
            ExtRangeFilterType(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum RangeFilterTypeError {
    Empty,
}

impl FromStr for RangeFilterType {
    type Err = RangeFilterTypeError;
    fn from_str(s: &str) -> Result<RangeFilterType, RangeFilterTypeError> {
        if s == "" {
            Err(RangeFilterTypeError::Empty)
        } else {
            Ok(match s {
                "include" | "Include" | "INCLUDE" => Include,
                "exclude" | "Exclude" | "EXCLUDE" => Exclude,
                _ => ExtRangeFilterType(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for RangeFilterType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Include => "include",
            Exclude => "exclude",
            ExtRangeFilterType(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(RangeFilterType::Include,
                   FromStr::from_str("include").unwrap());
        assert_eq!(RangeFilterType::Include,
                   FromStr::from_str("Include").unwrap());
        assert_eq!(RangeFilterType::Include,
                   FromStr::from_str("INCLUDE").unwrap());
        assert_eq!(RangeFilterType::Exclude,
                   FromStr::from_str("exclude").unwrap());
        assert_eq!(RangeFilterType::Exclude,
                   FromStr::from_str("Exclude").unwrap());
        assert_eq!(RangeFilterType::Exclude,
                   FromStr::from_str("EXCLUDE").unwrap());
        assert_eq!(RangeFilterType::ExtRangeFilterType("ADDED_TYPE".to_owned()),
                   FromStr::from_str("ADDED_TYPE").unwrap());
        let x: Result<RangeFilterType, _> = FromStr::from_str("");
        if let Err(RangeFilterTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("include".to_owned(),
                   format!("{}", RangeFilterType::Include));
        assert_eq!("ADDED_TYPE".to_owned(),
                   format!("{}",
                           RangeFilterType::ExtRangeFilterType("ADDED_TYPE".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(RangeFilterType::Include.as_ref(), "include");
        assert_eq!(RangeFilterType::Exclude.as_ref(), "exclude");
        assert_eq!(RangeFilterType::ExtRangeFilterType("ADDED_TYPE".to_owned()).as_ref(),
                   "ADDED_TYPE");
    }
}
