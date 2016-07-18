use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::DrilldownUsable::{Select, Extension};
#[cfg(feature="sharding")]
use self::DrilldownUsable::LogicalSelect;

#[derive (Clone, PartialEq, Eq, Debug)]
enum DrilldownUsable {
    Select,
    #[cfg(feature="sharding")]
    LogicalSelect,
    Extension(String),
}

#[derive (Debug)]
pub enum DrilldownUsableError {
    Empty,
}

impl AsRef<str> for DrilldownUsable {
    fn as_ref(&self) -> &str {
        match *self {
            Select => "select",
            #[cfg(feature="sharding")]
            LogicalSelect => "logical_select",
            Extension(ref s) => s.as_ref(),
        }
    }
}

impl FromStr for DrilldownUsable {
    type Err = DrilldownUsableError;
    fn from_str(s: &str) -> Result<DrilldownUsable, DrilldownUsableError> {
        if s == "" {
            Err(DrilldownUsableError::Empty)
        } else {
            Ok(match s {
                "select" => Select,
                #[cfg(feature="sharding")]
                "logical_select" => LogicalSelect,
                _ => Extension(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for DrilldownUsable {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Select => "select",
            #[cfg(feature="sharding")]
            LogicalSelect => "logical_select",
            Extension(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::DrilldownUsableError;
    use super::DrilldownUsable;
    use super::DrilldownUsable::{Select, Extension};
    #[cfg(feature="sharding")]
    use super::DrilldownUsable::LogicalSelect;

    #[test]
    fn test_from_str() {
        assert_eq!(Select, FromStr::from_str("select").unwrap());
        assert_eq!(Extension("added-command".to_owned()),
                   FromStr::from_str("added-command").unwrap());
        let x: Result<DrilldownUsable, _> = FromStr::from_str("");
        if let Err(DrilldownUsableError::Empty) = x {
        } else {
            panic!("An empty drilldown usable type string is invalid!")
        }
    }

    #[test]
    #[cfg(feature="sharding")]
    fn test_from_str_with_sharding() {
        assert_eq!(LogicalSelect, FromStr::from_str("logical_select").unwrap());
    }

    #[test]
    fn test_fmt() {
        assert_eq!("select".to_owned(), format!("{}", Select));
        assert_eq!("added-command".to_owned(),
                   format!("{}", Extension("added-command".to_owned())));
    }

    #[test]
    #[cfg(feature="sharding")]
    fn test_fmt_with_sharding() {
        assert_eq!("logical_select".to_owned(), format!("{}", LogicalSelect));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Select.as_ref(), "select");
        assert_eq!(Extension("added-command".to_owned()).as_ref(),
                   "added-command");
    }

    #[cfg(feature="sharding")]
    #[test]
    fn test_as_str_with_sharding() {
        assert_eq!(LogicalSelect.as_ref(), "logical_select");
    }
}
