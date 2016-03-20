use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::CalcType::{None, Count, Max, Min, Sum, Avg, ExtCalcType};

#[derive (Debug)]
pub enum CalcTypeError {
    Empty,
}

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum CalcType {
    None,
    Count,
    Max,
    Min,
    Sum,
    Avg,
    /// For future extensibility.
    ExtCalcType(String),
}

impl AsRef<str> for CalcType {
    fn as_ref(&self) -> &str {
        match *self {
            None => "NONE",
            Count => "COUNT",
            Max => "MAX",
            Min => "MIN",
            Sum => "SUM",
            Avg => "AVG",
            ExtCalcType(ref s) => s.as_ref(),
        }
    }
}

impl fmt::Display for CalcType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            None => "NONE",
            Count => "COUNT",
            Max => "MAX",
            Min => "MIN",
            Sum => "SUM",
            Avg => "AVG",
            ExtCalcType(ref s) => s.as_ref(),
        })
    }
}

impl FromStr for CalcType {
    type Err = CalcTypeError;
    fn from_str(s: &str) -> Result<CalcType, CalcTypeError> {
        if s == "" {
            Err(CalcTypeError::Empty)
        } else {
            Ok(match s {
                "None" | "NONE" => None,
                "Count" | "COUNT" => Count,
                "Max" | "MAX" => Max,
                "Min" | "MIN" => Min,
                "Sum" | "SUM" => Sum,
                "Avg" | "AVG" => Avg,
                _ => ExtCalcType(s.to_owned()),
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
        assert_eq!(CalcType::None, FromStr::from_str("None").unwrap());
        assert_eq!(CalcType::Count, FromStr::from_str("Count").unwrap());
        assert_eq!(CalcType::Max, FromStr::from_str("Max").unwrap());
        assert_eq!(CalcType::ExtCalcType("AddedType".to_owned()),
                   FromStr::from_str("AddedType").unwrap());
        let x: Result<CalcType, _> = FromStr::from_str("");
        if let Err(CalcTypeError::Empty) = x {
        } else {
            panic!("An empty calc type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("NONE".to_owned(), format!("{}", CalcType::None));
        assert_eq!("COUNT".to_owned(), format!("{}", CalcType::Count));
        assert_eq!("MAX".to_owned(), format!("{}", CalcType::Max));
        assert_eq!("MIN".to_owned(), format!("{}", CalcType::Min));
        assert_eq!("SUM".to_owned(), format!("{}", CalcType::Sum));
        assert_eq!("AVG".to_owned(), format!("{}", CalcType::Avg));
        assert_eq!("AddedType".to_owned(),
                   format!("{}", CalcType::ExtCalcType("AddedType".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(CalcType::None.as_ref(), "NONE");
        assert_eq!(CalcType::ExtCalcType("AddedType".to_owned()).as_ref(),
                   "AddedType");
    }
}
