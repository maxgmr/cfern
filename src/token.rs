use std::fmt::Display;

use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// A token of C code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub(crate) kind: TokenKind<'a>,
    len: usize,
    index: usize,
}
impl<'a> Token<'a> {
    /// Create an empty placeholder token.
    #[must_use]
    pub fn create_placeholder() -> Self {
        Self::new(TokenKind::Identifier(""), 0)
    }

    /// Create a new token.
    #[must_use]
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
        T: Fn(&'a str) -> Option<TokenKind<'_>>,
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

    /// Get the [`TokenKind`] of this token.
    #[must_use]
    pub fn kind(&self) -> &TokenKind<'_> {
        &self.kind
    }

    /// Get the index of this token.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get the length of this token.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check whether or not this token is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Convert to a [`String`] useful for error reporting.
    #[must_use]
    pub fn to_debug_string(&self) -> String {
        use TokenKind::{Constant, Identifier, Keyword, Symbol};

        match self.kind {
            Keyword(kw) => format!("keyword \"{}\" at index {}", kw, self.index),
            Identifier(i) => format!("identifier \"{}\" at index {}", i, self.index),
            Constant(c) => format!("constant `{}` at index {}", c, self.index),
            Symbol(s) => format!("symbol `{}` at index {}", s, self.index),
        }
    }
}

/// A particular kind of C code token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind<'a> {
    /// A C keyword (`int`, `void`, `return`, etc.).
    Keyword(Keyword),
    /// A C identifier (variable name, function name, etc.).
    Identifier(&'a str),
    /// A C integer constant.
    Constant(&'a str),
    /// A single-character symbol (parenthesis, brace, semicolon, etc.).
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
    /// `(`
    OpenParenthesis,
    /// `)`
    CloseParenthesis,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `;`
    Semicolon,
}
impl From<Symbol> for &'static str {
    fn from(value: Symbol) -> Self {
        (&value).into()
    }
}
impl From<&Symbol> for &'static str {
    fn from(value: &Symbol) -> Self {
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
        // OK to unwrap here; we know that the value isn't empty.
        #[allow(clippy::unwrap_used)]
        s.chars().next().unwrap()
    }
}
impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{s}")
    }
}

/// A C keyword.
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
#[allow(missing_docs)]
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
impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{s}")
    }
}

/// This error is returned if the given string slice doesn't match a C keyword.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct KeywordFromStrError<'a>(&'a str);
impl Display for KeywordFromStrError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "str '{}' does not match any C keyword", self.0)
    }
}
impl std::error::Error for KeywordFromStrError<'_> {}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
        let result: Result<Keyword, KeywordFromStrError<'_>> = s.as_str().try_into();
        assert_eq!(KeywordFromStrError("hello"), result.unwrap_err());

        let s = String::from("Signed");
        let result: Result<Keyword, KeywordFromStrError<'_>> = s.as_str().try_into();
        assert_eq!(KeywordFromStrError("Signed"), result.unwrap_err());

        let s = String::from(" int");
        let result: Result<Keyword, KeywordFromStrError<'_>> = s.as_str().try_into();
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
