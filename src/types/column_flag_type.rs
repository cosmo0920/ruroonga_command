use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::ColumnFlagType::{Scalar, Vector, Index, WithSection, WithWeight,
                           WithPosition, ExtColumnFlagType};
#[cfg(feature="unstable")]
use self::ColumnFlagType::{CompressZlib, CompressLz4};


#[derive (Clone, PartialEq, Eq, Debug)]
pub enum ColumnFlagType {
    Scalar,
    Vector,
    Index,
    WithSection,
    WithWeight,
    WithPosition,
    #[cfg(feature="unstable")]
    CompressZlib,
    #[cfg(feature="unstable")]
    CompressLz4,
    /// For future extensibility.
    ExtColumnFlagType(String),
}

impl AsRef<str> for ColumnFlagType {
    fn as_ref(&self) -> &str {
        match *self {
            Scalar => "COLUMN_SCALAR",
            Vector => "COLUMN_VECTOR",
            Index => "COLUMN_INDEX",
            WithSection => "WITH_SECTION",
            WithWeight =>  "WITH_WEIGHT",
            WithPosition => "WITH_POSITION",
            #[cfg(feature="unstable")]
            CompressZlib => "COMPRESS_ZLIB",
            #[cfg(feature="unstable")]
            CompressLz4 => "COMPRESS_LZ4",
            ExtColumnFlagType(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum ColumnFlagTypeError { Empty }

impl FromStr for ColumnFlagType {
    type Err = ColumnFlagTypeError;
    fn from_str(s: &str) -> Result<ColumnFlagType, ColumnFlagTypeError> {
        if s == "" {
            Err(ColumnFlagTypeError::Empty)
        } else {
            Ok(match s {
                "Scalar" | "ColumnScalar" | "COLUMN_SCALAR" => Scalar,
                "Vector" | "ColumnVector" | "COLUMN_VECTOR" => Vector,
                "Index" | "ColumnsIndex" | "COLUMN_INDEX" => Index,
                "WithSection" | "WITH_SECTION" => WithSection,
                "WithWeight" | "WITH_WEIGHT" => WithWeight,
                "WithPosition" | "WITH_POSITION" => WithPosition,
                #[cfg(feature="unstable")]
                "Zlib" | "CompressZlib" | "COMPRESS_ZLIB" => CompressZlib,
                #[cfg(feature="unstable")]
                "Lz4" | "CompressLz4" | "COMPRESS_LZ4" => CompressLz4,
                _ => ExtColumnFlagType(s.to_owned())
            })
        }
    }
}

impl fmt::Display for ColumnFlagType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Scalar => "COLUMN_SCALAR",
            Vector => "COLUMN_VECTOR",
            Index => "COLUMN_INDEX",
            WithSection => "WITH_SECTION",
            WithWeight =>  "WITH_WEIGHT",
            WithPosition => "WITH_POSITION",
            #[cfg(feature="unstable")]
            CompressZlib => "COMPRESS_ZLIB",
            #[cfg(feature="unstable")]
            CompressLz4 => "COMPRESS_LZ4",
            ExtColumnFlagType(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(ColumnFlagType::Scalar, FromStr::from_str("Scalar").unwrap());
        assert_eq!(ColumnFlagType::Scalar, FromStr::from_str("ColumnScalar").unwrap());
        assert_eq!(ColumnFlagType::ExtColumnFlagType("AddedColumnFlag".to_owned()),
                   FromStr::from_str("AddedColumnFlag").unwrap());
        let x: Result<ColumnFlagType, _> = FromStr::from_str("");
        if let Err(ColumnFlagTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    #[cfg(feature="unstable")]
    fn test_from_str_unstable() {
        assert_eq!(ColumnFlagType::CompressZlib,
                   FromStr::from_str("Zlib").unwrap());
        assert_eq!(ColumnFlagType::CompressLz4,
                   FromStr::from_str("CompressLz4").unwrap());
    }

    #[test]
    fn test_fmt() {
        assert_eq!("COLUMN_VECTOR".to_owned(), format!("{}", ColumnFlagType::Vector));
        assert_eq!("AddedColumnFlag".to_owned(),
                   format!("{}", ColumnFlagType::ExtColumnFlagType("AddedColumnFlag".to_owned())));
    }

    #[test]
    #[cfg(feature="unstable")]
    fn test_fmt_unstable() {
        assert_eq!("COMPRESS_ZLIB".to_owned(),
                   format!("{}", ColumnFlagType::CompressZlib));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(ColumnFlagType::Index.as_ref(), "COLUMN_INDEX");
        assert_eq!(ColumnFlagType::ExtColumnFlagType("AddedColumnFlag".to_owned()).as_ref(),
                   "AddedColumnFlag");
    }

    #[test]
    #[cfg(feature="unstable")]
    fn test_as_str_unstable() {
        assert_eq!(ColumnFlagType::CompressLz4.as_ref(),
                   "COMPRESS_LZ4");
    }
}
