use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::LogLevelType::{Debug, Info, Notice, Error, Warn, Critical, Alert, Emergency,
                         ExtLogLevelType};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum LogLevelType {
    Debug,
    Info,
    Notice,
    Error,
    Warn,
    Critical,
    Alert,
    Emergency,
    /// For future extensibility.
    ExtLogLevelType(String),
}

impl AsRef<str> for LogLevelType {
    fn as_ref(&self) -> &str {
        match *self {
            Debug => "debug",
            Info => "info",
            Notice => "notice",
            Error => "error",
            Warn => "warn",
            Critical => "critial",
            Alert => "alert",
            Emergency => "emergency",
            ExtLogLevelType(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum LogLevelTypeError {
    Empty,
}

impl FromStr for LogLevelType {
    type Err = LogLevelTypeError;
    fn from_str(s: &str) -> Result<LogLevelType, LogLevelTypeError> {
        if s == "" {
            Err(LogLevelTypeError::Empty)
        } else {
            Ok(match s {
                "debug" | "Debug" | "DEBUG" => Debug,
                "info" | "Info" | "INFO" => Info,
                "notice" | "Notice" | "NOTICE" => Notice,
                "error" | "Error" | "ERROR" => Error,
                "warn" | "Warn" | "WARN" => Warn,
                "critical" | "Critical" | "CRITICAL" | "CRIT" => Critical,
                "alert" | "Alert" | "ALERT" => Alert,
                "emergency" | "Emergency" | "EMERGENCY" | "EMERG" => Emergency,
                _ => ExtLogLevelType(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for LogLevelType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Debug => "debug",
            Info => "info",
            Notice => "notice",
            Error => "error",
            Warn => "warn",
            Critical => "critial",
            Alert => "alert",
            Emergency => "emergency",
            ExtLogLevelType(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(LogLevelType::Debug, FromStr::from_str("debug").unwrap());
        assert_eq!(LogLevelType::Debug, FromStr::from_str("Debug").unwrap());
        assert_eq!(LogLevelType::Debug, FromStr::from_str("DEBUG").unwrap());
        assert_eq!(LogLevelType::ExtLogLevelType("ADDED_TYPE".to_owned()),
                   FromStr::from_str("ADDED_TYPE").unwrap());
        let x: Result<LogLevelType, _> = FromStr::from_str("");
        if let Err(LogLevelTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("info".to_owned(), format!("{}", LogLevelType::Info));
        assert_eq!("ADDED_TYPE".to_owned(),
                   format!("{}", LogLevelType::ExtLogLevelType("ADDED_TYPE".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(LogLevelType::Alert.as_ref(), "alert");
        assert_eq!(LogLevelType::ExtLogLevelType("ADDED_TYPE".to_owned()).as_ref(),
                   "ADDED_TYPE");
    }
}
