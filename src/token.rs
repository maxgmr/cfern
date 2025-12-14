//! Tokens of C code.

use std::ops::Range;

use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// A token of C code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    /// The semantic content of this token
    pub(crate) kind: TokenKind<'a>,
    /// The location of this token in the source
    pub(crate) span: Span,
}
impl<'a> Token<'a> {
    /// Create a new token.
    #[must_use]
    pub fn new(kind: TokenKind<'a>, start: usize) -> Self {
        let len = kind.len();
        Self {
            kind,
            span: Span { start, len },
        }
    }

    /// Get the length of this token in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.span.len
    }

    /// Check whether or not this token is empty. This should never happen in practice.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.span.len == 0
    }
}
impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at byte {}", self.kind, self.span.start)
    }
}

/// The semantic content of a [`Token`].
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
impl<'a> TokenKind<'a> {
    /// Calculate the length of this token kind in bytes.
    #[must_use]
    fn len(&self) -> usize {
        match self {
            TokenKind::Keyword(kw) => {
                let s: &str = kw.into();
                s.len()
            }
            TokenKind::Identifier(s) | TokenKind::Constant(s) => s.len(),
            TokenKind::Symbol(_) => 1,
        }
    }

    /// Check whether or not this token kind is empty.
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check whether or not this token kind is a keyword.
    #[must_use]
    fn is_keyword(&self) -> bool {
        matches!(self, TokenKind::Keyword(_))
    }

    /// Check whether or not this token kind is an identifier.
    #[must_use]
    fn is_identifier(&self) -> bool {
        matches!(self, TokenKind::Identifier(_))
    }

    /// Check whether or not this token kind is a constant.
    #[must_use]
    fn is_constant(&self) -> bool {
        matches!(self, TokenKind::Constant(_))
    }

    /// Check whether or not this token kind is a symbol.
    #[must_use]
    fn is_symbol(&self) -> bool {
        matches!(self, TokenKind::Symbol(_))
    }
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
impl std::fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Keyword(kw) => write!(f, "keyword '{kw}'"),
            TokenKind::Identifier(id) => write!(f, "identifier '{id}'"),
            TokenKind::Constant(c) => write!(f, "constant '{c}'"),
            TokenKind::Symbol(s) => write!(f, "symbol '{s}'"),
        }
    }
}

/// The location information of a [`Token`] in the source.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    /// Starting byte position in the source
    pub(crate) start: usize,
    /// Length in bytes
    pub(crate) len: usize,
}
impl Span {
    /// Create a new span.
    #[must_use]
    fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    /// Get the end position (exclusive) of this span.
    #[must_use]
    fn end(&self) -> usize {
        self.start + self.len
    }

    /// Get the [`Range`] of this span.
    #[must_use]
    fn range(&self) -> Range<usize> {
        self.start..self.end()
    }

    /// Check whether or not this span contains a given position.
    #[must_use]
    fn contains(&self, position: usize) -> bool {
        position >= self.start && position < self.end()
    }

    /// Extract the text for this span from the source.
    #[must_use]
    fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.range()]
    }
}

/// An individual symbol with meaning in C.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
pub enum Symbol {
    #[strum(serialize = "(")]
    OpenParenthesis,
    #[strum(serialize = ")")]
    CloseParenthesis,
    #[strum(serialize = "{")]
    OpenBrace,
    #[strum(serialize = "}")]
    CloseBrace,
    #[strum(serialize = ";")]
    Semicolon,
}
impl From<Symbol> for char {
    fn from(value: Symbol) -> Self {
        let s: &'static str = value.into();
        s.chars().next().expect("Symbol string should not be empty")
    }
}
impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{s}")
    }
}

/// A C keyword.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
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
impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{s}")
    }
}

/// This error is returned if the given string slice doesn't match a C keyword.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct KeywordFromStrError<'a>(&'a str);
impl std::fmt::Display for KeywordFromStrError<'_> {
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
