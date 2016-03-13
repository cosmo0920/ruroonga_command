use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::NormalizerType::{Auto, NFKC51, ExtNormalizerType};
#[cfg(feature="normalizer_mysql")]
use self::NormalizerType::{MySQLGeneralCI, MySQLUnicodeCI, MySQLUnicode520CI,
                           MySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark,
                           MySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum NormalizerType {
    Auto,
    NFKC51,
    #[cfg(feature="normalizer_mysql")]
    MySQLGeneralCI,
    #[cfg(feature="normalizer_mysql")]
    MySQLUnicodeCI,
    #[cfg(feature="normalizer_mysql")]
    MySQLUnicode520CI,
    #[cfg(feature="normalizer_mysql")]
    MySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark,
    #[cfg(feature="normalizer_mysql")]
    MySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark,
    ExtNormalizerType(String),
}


impl AsRef<str> for NormalizerType {
    fn as_ref(&self) -> &str {
        match *self {
            Auto => "NormalizerAuto",
            NFKC51 => "NormalizerNFKC51",
            #[cfg(feature="normalizer_mysql")]
            MySQLGeneralCI => "NormalizerMySQLGeneralCI",
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicodeCI => "NormalizerMySQLUnicodeCI",
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicode520CI => "NormalizerMySQLUnicode520CI",
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark => {
                "NormalizerMySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark"
            }
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark => {
                "NormalizerMySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark"
            }
            ExtNormalizerType(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum NormalizerTypeError {
    Empty,
}

impl FromStr for NormalizerType {
    type Err = NormalizerTypeError;
    fn from_str(s: &str) -> Result<NormalizerType, NormalizerTypeError> {
        if s == "" {
            Err(NormalizerTypeError::Empty)
        } else {
            Ok(match s {
                "Auto" | "NormalizerAuto" => Auto,
                "NFKC51" | "NormalizerNFKC51" => NFKC51,
                #[cfg(feature="normalizer_mysql")]
                "MySQLGeneralCI" | "NormalizerMySQLGeneralCI" => MySQLGeneralCI,
                #[cfg(feature="normalizer_mysql")]
                "MySQLUnicodeCI" | "NormalizerMySQLUnicodeCI" => MySQLUnicodeCI,
                #[cfg(feature="normalizer_mysql")]
                "MySQLUnicode520CI" | "NormalizerMySQLUnicode520CI" => MySQLUnicode520CI,
                #[cfg(feature="normalizer_mysql")]
                "MySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark" |
                "NormalizerMySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark" => {
                    MySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark
                }
                #[cfg(feature="normalizer_mysql")]
                "MySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark" |
                "NormalizerMySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark" => {
                    MySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark
                }
                _ => ExtNormalizerType(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for NormalizerType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Auto => "NormalizerAuto",
            NFKC51 => "NormalizerNFKC51",
            #[cfg(feature="normalizer_mysql")]
            MySQLGeneralCI => "NormalizerMySQLGeneralCI",
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicodeCI => "NormalizerMySQLUnicodeCI",
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicode520CI => "NormalizerMySQLUnicode520CI",
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark => {
                "NormalizerMySQLUnicodeCIExceptKanaCIKanaWithVoicedSoundMark"
            }
            #[cfg(feature="normalizer_mysql")]
            MySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark => {
                "NormalizerMySQLUnicode520CIExceptKanaCIKanaWithVoicedSoundMark"
            }
            ExtNormalizerType(ref s) => s.as_ref(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(NormalizerType::Auto, FromStr::from_str("Auto").unwrap());
        assert_eq!(NormalizerType::Auto,
                   FromStr::from_str("NormalizerAuto").unwrap());
        assert_eq!(NormalizerType::ExtNormalizerType("AddedNormalizer".to_owned()),
                   FromStr::from_str("AddedNormalizer").unwrap());
        let x: Result<NormalizerType, _> = FromStr::from_str("");
        if let Err(NormalizerTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    #[cfg(feature="normalizer_mysql")]
    fn test_from_str_normalizer_mysql() {
        assert_eq!(NormalizerType::MySQLUnicodeCI,
                   FromStr::from_str("MySQLUnicodeCI").unwrap());
        assert_eq!(NormalizerType::MySQLUnicodeCI,
                   FromStr::from_str("NormalizerMySQLUnicodeCI").unwrap());
    }

    #[test]
    fn test_fmt() {
        assert_eq!("NormalizerNFKC51".to_owned(),
                   format!("{}", NormalizerType::NFKC51));
        assert_eq!("AddedNormalizer".to_owned(),
                   format!("{}",
                           NormalizerType::ExtNormalizerType("AddedNormalizer".to_owned())));
    }

    #[test]
    #[cfg(feature="normalizer_mysql")]
    fn test_fmt_normalizer_mysql() {
        assert_eq!("NormalizerMySQLUnicodeCI".to_owned(),
                   format!("{}", NormalizerType::MySQLUnicodeCI));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(NormalizerType::NFKC51.as_ref(), "NormalizerNFKC51");
        assert_eq!(NormalizerType::ExtNormalizerType("AddedNormalizer".to_owned()).as_ref(),
                   "AddedNormalizer");
    }

    #[test]
    #[cfg(feature="normalizer_mysql")]
    fn test_as_str_normalizer_mysql() {
        assert_eq!(NormalizerType::MySQLGeneralCI.as_ref(),
                   "NormalizerMySQLGeneralCI");
    }
}
