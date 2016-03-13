use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::TokenizerType::{Bigram, BigramSplitSymbol, BigramSplitSymbolAlpha,
                          BigramSplitSymbolAlphaDigit, BigramIgnoreBlank,
                          BigramIgnoreBlankSplitSymbol, BigramIgnoreBlankSplitAlpha,
                          BigramIgnoreBlankSplitAlphaDigit, Unigram, Trigram, Delimit,
                          DelimitNull, Mecab, Regexp, ExtUserDefined};

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum TokenizerType {
    Bigram,
    BigramSplitSymbol,
    BigramSplitSymbolAlpha,
    BigramSplitSymbolAlphaDigit,
    BigramIgnoreBlank,
    BigramIgnoreBlankSplitSymbol,
    BigramIgnoreBlankSplitAlpha,
    BigramIgnoreBlankSplitAlphaDigit,
    Unigram,
    Trigram,
    Delimit,
    DelimitNull,
    Mecab,
    Regexp,
    /// Groonga can introduce user-defined tokenizer as a its plugin.
    ExtUserDefined(String),
}

impl AsRef<str> for TokenizerType {
    fn as_ref(&self) -> &str {
        match *self {
            Bigram => "TokenBigram",
            BigramSplitSymbol => "TokenBigramSplitSymbol",
            BigramSplitSymbolAlpha => "TokenBigramSplitSymbolAlpha",
            BigramSplitSymbolAlphaDigit => "TokenBigramSplitSymbolAlphaDigit",
            BigramIgnoreBlank => "TokenBigramIgnoreBlank",
            BigramIgnoreBlankSplitSymbol => "TokenBigramIgnoreBlankSplitSymbol",
            BigramIgnoreBlankSplitAlpha => "TokenBigramIgnoreBlankSplitAlpha",
            BigramIgnoreBlankSplitAlphaDigit => "TokenBigramIgnoreBlankSplitAlphaDigit",
            Unigram => "TokenUnigram",
            Trigram => "TokenTrigram",
            Delimit => "TokenDelimit",
            DelimitNull => "TokenDelimitNull",
            Mecab => "TokenMecab",
            Regexp => "TokenRegexp",
            ExtUserDefined(ref s) => s.as_ref(),
        }
    }
}

#[derive(Debug)]
pub enum TokenizerTypeError {
    Empty,
}

impl FromStr for TokenizerType {
    type Err = TokenizerTypeError;
    fn from_str(s: &str) -> Result<TokenizerType, TokenizerTypeError> {
        if s == "" {
            Err(TokenizerTypeError::Empty)
        } else {
            Ok(match s {
                "TokenBigram" | "Bigram" => Bigram,
                "TokenBigramSplitSymbol" | "BigramSplitSymbol" => BigramSplitSymbol,
                "TokenBigramSplitSymbolAlpha" | "BigramSplitSymbolAlpha" => BigramSplitSymbolAlpha,
                "TokenBigramSplitSymbolAlphaDigit" | "BigramSplitSymbolAlphaDigit" => {
                    BigramSplitSymbolAlphaDigit
                }
                "TokenBigramIgnoreBlank" | "BigramIgnoreBlank" => BigramIgnoreBlank,
                "TokenBigramIgnoreBlankSplitSymbol" | "BigramIgnoreBlankSplitSymbol" => {
                    BigramIgnoreBlankSplitSymbol
                }
                "TokenBigramIgnoreBlankSplitAlpha" | "BigramIgnoreBlankSplitAlpha" => {
                    BigramIgnoreBlankSplitAlpha
                }
                "TokenBigramIgnoreBlankSplitAlphaDigit" | "BigramIgnoreBlankSplitAlphaDigit" => {
                    BigramIgnoreBlankSplitAlphaDigit
                }
                "TokenUnigram" | "Unigram" => Unigram,
                "TokenTrigram" | "Trigram" => Trigram,
                "TokenDelimit" | "Delimit" => Delimit,
                "TokenDelimitNull" | "DelimitNull" => DelimitNull,
                "TokenMecab" | "Mecab" => Mecab,
                "TokenRegexp" | "Regexp" => Regexp,
                _ => ExtUserDefined(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for TokenizerType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Bigram => "TokenBigram",
            BigramSplitSymbol => "TokenBigramSplitSymbol",
            BigramSplitSymbolAlpha => "TokenBigramSplitSymbolAlpha",
            BigramSplitSymbolAlphaDigit => "TokenBigramSplitSymbolAlphaDigit",
            BigramIgnoreBlank => "TokenBigramIgnoreBlank",
            BigramIgnoreBlankSplitSymbol => "TokenBigramIgnoreBlankSplitSymbol",
            BigramIgnoreBlankSplitAlpha => "TokenBigramIgnoreBlankSplitAlpha",
            BigramIgnoreBlankSplitAlphaDigit => "TokenBigramIgnoreBlankSplitAlphaDigit",
            Unigram => "TokenUnigram",
            Trigram => "TokenTrigram",
            Delimit => "TokenDelimit",
            DelimitNull => "TokenDelimitNull",
            Mecab => "TokenMecab",
            Regexp => "TokenRegexp",
            ExtUserDefined(ref s) => s.as_ref(),
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(TokenizerType::Bigram, FromStr::from_str("Bigram").unwrap());
        assert_eq!(TokenizerType::Bigram,
                   FromStr::from_str("TokenBigram").unwrap());
        assert_eq!(TokenizerType::ExtUserDefined("ADDED_TYPE".to_owned()),
                   FromStr::from_str("ADDED_TYPE").unwrap());
        let x: Result<TokenizerType, _> = FromStr::from_str("");
        if let Err(TokenizerTypeError::Empty) = x {
        } else {
            panic!("An empty table type is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("TokenBigramSplitSymbolAlphaDigit".to_owned(),
                   format!("{}", TokenizerType::BigramSplitSymbolAlphaDigit));
        assert_eq!("ADDED_TYPE".to_owned(),
                   format!("{}", TokenizerType::ExtUserDefined("ADDED_TYPE".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(TokenizerType::Trigram.as_ref(), "TokenTrigram");
        assert_eq!(TokenizerType::ExtUserDefined("ADDED_TYPE".to_owned()).as_ref(),
                   "ADDED_TYPE");
    }
}
