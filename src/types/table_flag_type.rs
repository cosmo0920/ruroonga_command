use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::TableFlagType::{NoKey, HashKey, PatKey, DatKey, KeyWithSIS, ExtTableFlagType};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum TableFlagType {
    NoKey,
    HashKey,
    PatKey,
    DatKey,
    KeyWithSIS,
    /// For future extensibility.
    ExtTableFlagType(String),
}

impl AsRef<str> for TableFlagType {
    fn as_ref(&self) -> &str {
        match *self {
            NoKey => "TABLE_NO_KEY",
            HashKey => "TABLE_HASH_KEY",
            PatKey => "TABLE_PAT_KEY",
            DatKey => "TABLE_DAT_KEY",
            KeyWithSIS => "TABLE_KEY_WITH_SIS",
            ExtTableFlagType(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum TableFlagTypeError {
    Empty,
}

impl FromStr for TableFlagType {
    type Err = TableFlagTypeError;
    fn from_str(s: &str) -> Result<TableFlagType, TableFlagTypeError> {
        if s == "" {
            Err(TableFlagTypeError::Empty)
        } else {
            Ok(match s {
                "TABLE_NO_KEY" | "NO_KEY" | "NoKey" => NoKey,
                "TABLE_HASH_KEY" | "HASH_KEY" | "HashKey" => HashKey,
                "TABLE_PAT_KEY" | "PAT_KEY" | "PatKey" => PatKey,
                "TABLE_DAT_KEY" | "DAT_KEY" | "DatKey" => DatKey,
                "TABLE_KEY_WITH_SIS" | "KEY_WITH_SIS" | "WithSIS" => KeyWithSIS,
                _ => ExtTableFlagType(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for TableFlagType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            NoKey => "TABLE_NO_KEY",
            HashKey => "TABLE_HASH_KEY",
            PatKey => "TABLE_PAT_KEY",
            DatKey => "TABLE_DAT_KEY",
            KeyWithSIS => "TABLE_KEY_WITH_SIS",
            ExtTableFlagType(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(TableFlagType::NoKey, FromStr::from_str("NoKey").unwrap());
        assert_eq!(TableFlagType::NoKey, FromStr::from_str("NO_KEY").unwrap());
        assert_eq!(TableFlagType::NoKey,
                   FromStr::from_str("TABLE_NO_KEY").unwrap());
        assert_eq!(TableFlagType::ExtTableFlagType("ADDED_TYPE".to_owned()),
                   FromStr::from_str("ADDED_TYPE").unwrap());
        let x: Result<TableFlagType, _> = FromStr::from_str("");
        if let Err(TableFlagTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("TABLE_PAT_KEY".to_owned(),
                   format!("{}", TableFlagType::PatKey));
        assert_eq!("ADDED_TYPE".to_owned(),
                   format!("{}",
                           TableFlagType::ExtTableFlagType("ADDED_TYPE".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(TableFlagType::DatKey.as_ref(), "TABLE_DAT_KEY");
        assert_eq!(TableFlagType::ExtTableFlagType("ADDED_TYPE".to_owned()).as_ref(),
                   "ADDED_TYPE");
    }
}
