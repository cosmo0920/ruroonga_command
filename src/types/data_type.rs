use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::DataType::{Bool, Int8, UInt8, Int16, UInt16, Int32, UInt32, Int64, UInt64, Float, Time,
                     ShortText, Text, LongText, TokyoGeoPoint, WGS84GeoPoint, ExtDataType};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum DataType {
    Bool,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Time,
    ShortText,
    Text,
    LongText,
    TokyoGeoPoint,
    WGS84GeoPoint,
    /// For future extensibility.
    ExtDataType(String),
}

impl AsRef<str> for DataType {
    fn as_ref(&self) -> &str {
        match *self {
            Bool => "Bool",
            Int8 => "Int8",
            UInt8 => "UInt8",
            Int16 => "Int16",
            UInt16 => "UInt16",
            Int32 => "Int32",
            UInt32 => "UInt32",
            Int64 => "Int64",
            UInt64 => "UInt64",
            Float => "Float",
            Time => "Time",
            ShortText => "ShortText",
            Text => "Text",
            LongText => "LongText",
            TokyoGeoPoint => "TokyoGeoPoint",
            WGS84GeoPoint => "WGS84GeoPoint",
            /// For future extensibility.
            ExtDataType(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum DataTypeError {
    Empty,
}

impl FromStr for DataType {
    type Err = DataTypeError;
    fn from_str(s: &str) -> Result<DataType, DataTypeError> {
        if s == "" {
            Err(DataTypeError::Empty)
        } else {
            Ok(match s {
                "bool" | "Bool" => Bool,
                "i8" | "int8" | "Int8" => Int8,
                "u8" | "uint8" | "UInt8" => UInt8,
                "i16" | "int16" | "Int16" => Int16,
                "u16" | "uint16" | "UInt16" => UInt16,
                "i32" | "int32" | "Int32" => Int32,
                "u32" | "uint32" | "UInt32" => UInt32,
                "i64" | "int64" | "Int64" => Int64,
                "u64" | "uint64" | "UInt64" => UInt64,
                "float" | "Float" => Float,
                "time" | "Time" => Time,
                "short_text" | "ShortText" => ShortText,
                "text" | "Text" => Text,
                "long_text" | "LongText" => LongText,
                "TokyoGeoPoint" => TokyoGeoPoint,
                "WGS84GeoPoint" => WGS84GeoPoint,
                /// For future extensibility.
                _ => ExtDataType(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Bool => "Bool",
            Int8 => "Int8",
            UInt8 => "UInt8",
            Int16 => "Int16",
            UInt16 => "UInt16",
            Int32 => "Int32",
            UInt32 => "UInt32",
            Int64 => "Int64",
            UInt64 => "UInt64",
            Float => "Float",
            Time => "Time",
            ShortText => "ShortText",
            Text => "Text",
            LongText => "LongText",
            TokyoGeoPoint => "TokyoGeoPoint",
            WGS84GeoPoint => "WGS84GeoPoint",
            /// For future extensibility.
            ExtDataType(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(DataType::UInt16, FromStr::from_str("u16").unwrap());
        assert_eq!(DataType::UInt16, FromStr::from_str("uint16").unwrap());
        assert_eq!(DataType::UInt16, FromStr::from_str("UInt16").unwrap());
        assert_eq!(DataType::ExtDataType("AddedType".to_owned()),
                   FromStr::from_str("AddedType").unwrap());
        let x: Result<DataType, _> = FromStr::from_str("");
        if let Err(DataTypeError::Empty) = x {
        } else {
            panic!("An empty data type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("TokyoGeoPoint".to_owned(),
                   format!("{}", DataType::TokyoGeoPoint));
        assert_eq!("AddedType".to_owned(),
                   format!("{}", DataType::ExtDataType("AddedType".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(DataType::Time.as_ref(), "Time");
        assert_eq!(DataType::ExtDataType("UserDefined".to_owned()).as_ref(),
                   "UserDefined");
    }
}
