use std::fmt::Display;

use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// A token of C code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {}

/// A C keyword.
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}
impl<'a> TryFrom<&'a str> for Keyword {
    type Error = KeywordFromStrError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        for keyword in Keyword::iter() {
            let s: &'static str = keyword.into();
            if s == value {
                return Ok(keyword);
            }
        }
        Err(KeywordFromStrError(value))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct KeywordFromStrError<'a>(&'a str);
impl Display for KeywordFromStrError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "str '{}' does not match any C keyword", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_keyword_from_str() {
        assert_eq!(
            Keyword::Typedef,
            String::from("typedef").as_str().try_into().unwrap()
        );
        assert_eq!(
            Keyword::Volatile,
            String::from("volatile").as_str().try_into().unwrap()
        );
        assert_eq!(
            Keyword::Continue,
            String::from("continue").as_str().try_into().unwrap()
        );
    }

    #[test]
    fn try_keyword_from_str_no_match() {
        let s = String::from("hello");
        let result: Result<Keyword, KeywordFromStrError> = s.as_str().try_into();
        assert_eq!(KeywordFromStrError("hello"), result.unwrap_err());

        let s = String::from("Signed");
        let result: Result<Keyword, KeywordFromStrError> = s.as_str().try_into();
        assert_eq!(KeywordFromStrError("Signed"), result.unwrap_err());

        let s = String::from(" int");
        let result: Result<Keyword, KeywordFromStrError> = s.as_str().try_into();
        assert_eq!(KeywordFromStrError(" int"), result.unwrap_err());
    }

    #[test]
    fn keyword_as_str() {
        let s: &'static str = Keyword::Int.into();
        assert_eq!(s, "int");
    }
}
