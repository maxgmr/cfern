use std::{fmt::Display, sync::LazyLock};

use regex::Regex;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

const KEYWORD_REGEX: &str = r"^([a-zA-Z]+)(?-u:\b)";
const IDENTIFIER_REGEX: &str = r"^([a-zA-Z_][0-9A-Za-z_]*)(?-u:\b)";
const CONSTANT_REGEX: &str = r"^([0-9]+)(?-u:\b)";

/// Generates regex helper functions which statically create regexes for each [`Token`] variant.
macro_rules! generate_token_regexes {
    [
        $($variant:ident => $regex:expr),* $(,)?
    ] => {
        $(
            paste::paste! {
                pub fn [<match_ $variant:lower>](haystack: &str) -> Option<&str> {
                    static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new($regex).unwrap());
                    REGEX.captures(haystack)
                        .and_then(|caps| caps.get(1))
                        .map(|m| m.as_str())
                }
            }
        )*
    };
}
generate_token_regexes![
    Keyword => KEYWORD_REGEX,
    Identifier => IDENTIFIER_REGEX,
    Constant => CONSTANT_REGEX,
];

/// A token of C code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    len: usize,
    index: usize,
}
impl<'a> Token<'a> {
    /// Create an empty placeholder token.
    pub fn create_placeholder() -> Self {
        Self::new(TokenKind::Identifier(""), 0)
    }

    /// Create a new token.
    pub fn new(kind: TokenKind<'a>, index: usize) -> Self {
        let len = match kind {
            TokenKind::Keyword(keyword) => {
                let kw_s: &'a str = keyword.into();
                kw_s.len()
            }
            TokenKind::Identifier(ident) => ident.len(),
            TokenKind::Constant(constant) => constant.len(),
            TokenKind::Symbol(_) => 1,
        };
        Self { kind, len, index }
    }

    /// If the start of the provided data satisfies the `match_fn`, that matched substring
    /// satisfies the `token_fn`, and the length of the matched string is longer than the current
    /// token length, then update this token.
    pub fn try_update<S, T>(&mut self, data: &'a str, index: usize, match_fn: S, token_fn: T)
    where
        S: Fn(&'a str) -> Option<&'a str>,
        T: Fn(&'a str) -> Option<TokenKind>,
    {
        if let Some(new_str) = match_fn(data)
            && new_str.len() > self.len
            && let Some(token_kind) = token_fn(new_str)
        {
            *self = Self {
                kind: token_kind,
                len: new_str.len(),
                index,
            };
        }
    }

    /// Get the length of this token.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check whether or not this token is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// A particular kind of C code token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind<'a> {
    Keyword(Keyword),
    Identifier(&'a str),
    Constant(&'a str),
    Symbol(Symbol),
}
impl<'a> From<TokenKind<'a>> for &'a str {
    fn from(value: TokenKind<'a>) -> Self {
        match value {
            TokenKind::Keyword(keyword) => keyword.into(),
            TokenKind::Identifier(ident) => ident,
            TokenKind::Constant(constant) => constant,
            TokenKind::Symbol(symbol) => symbol.into(),
        }
    }
}

/// An individual symbol with meaning in C.
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter)]
pub enum Symbol {
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}
impl From<Symbol> for &'static str {
    fn from(value: Symbol) -> Self {
        match value {
            Symbol::OpenParenthesis => "(",
            Symbol::CloseParenthesis => ")",
            Symbol::OpenBrace => "{",
            Symbol::CloseBrace => "}",
            Symbol::Semicolon => ";",
        }
    }
}
impl From<Symbol> for char {
    fn from(value: Symbol) -> Self {
        let s: &'static str = value.into();
        s.chars().nth(0).unwrap()
    }
}

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
    fn keyword_regex() {
        assert_eq!(
            match_keyword("static int my_fn() { return 0; }"),
            Some("static")
        );
    }

    #[test]
    fn identifier_regex() {
        assert_eq!(match_identifier("my_var_123 = 6;"), Some("my_var_123"));
    }

    #[test]
    fn constant_regex() {
        assert_eq!(match_constant("24637;"), Some("24637"));
    }

    #[test]
    fn non_ascii_ident_regex() {
        assert_eq!(match_identifier("你好马 = \"good\";"), None);
    }

    #[test]
    fn non_ascii_end_ident_regex() {
        assert_eq!(match_identifier("test_1234五"), Some("test_1234"));
    }

    #[test]
    fn ident_regex_no_match() {
        assert_eq!(match_identifier("不好; int test = 0;"), None);
    }

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

    #[test]
    fn verify_symbol_to_char_ok() {
        for symbol in Symbol::iter() {
            let _: char = symbol.into();
        }
    }
}
