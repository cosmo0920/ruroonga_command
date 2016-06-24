use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::QueryFlagsType::{None, AllowPragma, AllowColumn, AllowUpdate, AllowLeadingNot,
                           ExtQueryFlagsType};

#[derive (Debug)]
pub enum QueryFlagsTypeError {
    Empty,
}

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum QueryFlagsType {
    None,
    AllowPragma,
    AllowColumn,
    AllowUpdate,
    AllowLeadingNot,
    /// For future extensibility.
    ExtQueryFlagsType(String),
}

impl AsRef<str> for QueryFlagsType {
    fn as_ref(&self) -> &str {
        match *self {
            None => "NONE",
            AllowPragma => "ALLOW_PRAGMA",
            AllowColumn => "ALLOW_COLUMN",
            AllowUpdate => "ALLOW_UPDATE",
            AllowLeadingNot => "ALLOW_LEADING_NOT",
            ExtQueryFlagsType(ref s) => s.as_ref(),
        }
    }
}

impl fmt::Display for QueryFlagsType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            None => "NONE",
            AllowPragma => "ALLOW_PRAGMA",
            AllowColumn => "ALLOW_COLUMN",
            AllowUpdate => "ALLOW_UPDATE",
            AllowLeadingNot => "ALLOW_LEADING_NOT",
            ExtQueryFlagsType(ref s) => s.as_ref(),
        })
    }
}

impl FromStr for QueryFlagsType {
    type Err = QueryFlagsTypeError;
    fn from_str(s: &str) -> Result<QueryFlagsType, QueryFlagsTypeError> {
        if s == "" {
            Err(QueryFlagsTypeError::Empty)
        } else {
            Ok(match s {
                "None" | "NONE" => None,
                "AllowPragma" | "ALLOW_PRAGMA" => AllowPragma,
                "AllowColumn" | "ALLOW_COLUMN" => AllowColumn,
                "AllowUpdate" | "ALLOW_UDATE" => AllowUpdate,
                "AllowLeadingNot" |
                "ALLOW_LEADING_NOT" => AllowLeadingNot,
                _ => ExtQueryFlagsType(s.to_owned()),
            })
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(QueryFlagsType::None, FromStr::from_str("None").unwrap());
        assert_eq!(QueryFlagsType::AllowPragma,
                   FromStr::from_str("AllowPragma").unwrap());
        assert_eq!(QueryFlagsType::AllowColumn,
                   FromStr::from_str("AllowColumn").unwrap());
        assert_eq!(QueryFlagsType::ExtQueryFlagsType("AddedType".to_owned()),
                   FromStr::from_str("AddedType").unwrap());
        let x: Result<QueryFlagsType, _> = FromStr::from_str("");
        if let Err(QueryFlagsTypeError::Empty) = x {
        } else {
            panic!("An empty calc type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("NONE".to_owned(), format!("{}", QueryFlagsType::None));
        assert_eq!("ALLOW_PRAGMA".to_owned(),
                   format!("{}", QueryFlagsType::AllowPragma));
        assert_eq!("ALLOW_COLUMN".to_owned(),
                   format!("{}", QueryFlagsType::AllowColumn));
        assert_eq!("ALLOW_UPDATE".to_owned(),
                   format!("{}", QueryFlagsType::AllowUpdate));
        assert_eq!("ALLOW_LEADING_NOT".to_owned(),
                   format!("{}", QueryFlagsType::AllowLeadingNot));
        assert_eq!("AddedType".to_owned(),
                   format!("{}",
                           QueryFlagsType::ExtQueryFlagsType("AddedType".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(QueryFlagsType::None.as_ref(), "NONE");
        assert_eq!(QueryFlagsType::ExtQueryFlagsType("AddedType".to_owned()).as_ref(),
                   "AddedType");
    }
}
