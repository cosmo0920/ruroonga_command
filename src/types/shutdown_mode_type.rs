use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::ShutdownModeType::{Graceful, Immediate, ExtShutdownModeType};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum ShutdownModeType {
    Graceful,
    Immediate,
    /// For future extensibility.
    ExtShutdownModeType(String),
}

impl AsRef<str> for ShutdownModeType {
    fn as_ref(&self) -> &str {
        match *self {
            Graceful => "graceful",
            Immediate => "immediate",
            ExtShutdownModeType(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum ShutdownModeTypeError {
    Empty,
}

impl FromStr for ShutdownModeType {
    type Err = ShutdownModeTypeError;
    fn from_str(s: &str) -> Result<ShutdownModeType, ShutdownModeTypeError> {
        if s == "" {
            Err(ShutdownModeTypeError::Empty)
        } else {
            Ok(match s {
                "graceful" | "Graceful" | "GRACEFUL" => Graceful,
                "immediate" | "Immediate" | "IMMEDIATE" => Immediate,
                _ => ExtShutdownModeType(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for ShutdownModeType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Graceful => "graceful",
            Immediate => "immediate",
            ExtShutdownModeType(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(ShutdownModeType::Graceful,
                   FromStr::from_str("graceful").unwrap());
        assert_eq!(ShutdownModeType::Graceful,
                   FromStr::from_str("Graceful").unwrap());
        assert_eq!(ShutdownModeType::Graceful,
                   FromStr::from_str("GRACEFUL").unwrap());
        assert_eq!(ShutdownModeType::ExtShutdownModeType("ADDED_TYPE".to_owned()),
                   FromStr::from_str("ADDED_TYPE").unwrap());
        let x: Result<ShutdownModeType, _> = FromStr::from_str("");
        if let Err(ShutdownModeTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("graceful".to_owned(),
                   format!("{}", ShutdownModeType::Graceful));
        assert_eq!("ADDED_TYPE".to_owned(),
                   format!("{}",
                           ShutdownModeType::ExtShutdownModeType("ADDED_TYPE".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(ShutdownModeType::Immediate.as_ref(), "immediate");
        assert_eq!(ShutdownModeType::ExtShutdownModeType("ADDED_TYPE".to_owned()).as_ref(),
                   "ADDED_TYPE");
    }
}
