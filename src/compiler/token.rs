use std::{fmt::Display, sync::LazyLock};

use regex::Regex;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

const KEYWORD_REGEX: &str = r"^([a-zA-Z]+)(?-u:\b)";
const IDENTIFIER_REGEX: &str = r"^([a-zA-Z_][0-9A-Za-z_]*)(?-u:\b)";
const CONSTANT_REGEX: &str = r"^([0-9]+)(?-u:\b)";

/// Returns the longest [`Token`] in the given `str`, along with the length of the matched
/// sequence (if any).
fn get_next_token<'a>(data: &'a str) -> Option<(Token<'a>, usize)> {
    #[derive(Debug)]
    struct TokenMatch<'a> {
        token: Option<Token<'a>>,
        len: usize,
    }
    impl<'a> TokenMatch<'a> {
        fn new() -> Self {
            Self {
                token: None,
                len: 0,
            }
        }

        fn try_update<S, T>(&mut self, data: &'a str, match_fn: S, token_fn: T)
        where
            S: Fn(&'a str) -> Option<&'a str>,
            T: Fn(&'a str) -> Option<Token>,
        {
            if let Some(new_str) = match_fn(data)
                && new_str.len() > self.len
                && let Some(token) = token_fn(new_str)
            {
                self.token = Some(token);
                self.len = new_str.len();
            }
        }
    }

    if data.is_empty() {
        return None;
    }

    let mut current_match = TokenMatch::new();

    // Try to match keyword
    current_match.try_update(data, match_keyword, |s| {
        Keyword::try_from(s).ok().map(Token::Keyword)
    });
    // Try to match ident
    current_match.try_update(data, match_identifier, |s| Some(Token::Identifier(s)));
    // Try to match constant
    current_match.try_update(data, match_constant, |s| Some(Token::Constant(s)));
    // If a match has been found, return it
    if let Some(token) = current_match.token {
        return Some((token, current_match.len));
    }

    // Try to match symbols
    for symbol in Symbol::iter() {
        let symbol_str: &'static str = symbol.into();
        if &data[..1] == symbol_str {
            return Some((Token::Symbol(symbol), 1));
        }
    }

    // Unable to match with Token
    None
}

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
pub enum Token<'a> {
    Keyword(Keyword),
    Identifier(&'a str),
    Constant(&'a str),
    Symbol(Symbol),
}
impl<'a> From<Token<'a>> for &'a str {
    fn from(value: Token<'a>) -> Self {
        match value {
            Token::Keyword(keyword) => keyword.into(),
            Token::Identifier(ident) => ident,
            Token::Constant(constant) => constant,
            Token::Symbol(symbol) => symbol.into(),
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

    #[test]
    fn next_token_keyword() {
        assert_eq!(
            get_next_token("int my_val = 0;"),
            Some((Token::Keyword(Keyword::Int), 3))
        )
    }

    #[test]
    fn next_token_ident() {
        assert_eq!(
            get_next_token("my_val = 0;"),
            Some((Token::Identifier("my_val"), 6))
        )
    }

    #[test]
    fn next_token_const() {
        assert_eq!(get_next_token("0;"), Some((Token::Constant("0"), 1)))
    }

    #[test]
    fn next_token_semicolon() {
        assert_eq!(
            get_next_token(";"),
            Some((Token::Symbol(Symbol::Semicolon), 1))
        )
    }

    #[test]
    fn next_token_empty() {
        assert_eq!(get_next_token(""), None);
    }

    #[test]
    fn next_token_actually_ident() {
        assert_eq!(get_next_token("inti"), Some((Token::Identifier("inti"), 4)))
    }
}
