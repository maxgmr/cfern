//! Matchers for different [`Token`]s.

use std::sync::LazyLock;

use regex::Regex;
use strum::IntoEnumIterator;

use crate::token::{Keyword, Symbol, Token, TokenKind};

const KEYWORD_REGEX: &str = r"^([a-zA-Z]+)(?-u:\b)";
const IDENTIFIER_REGEX: &str = r"^([a-zA-Z_][0-9A-Za-z_]*)(?-u:\b)";
const CONSTANT_REGEX: &str = r"^([0-9]+)(?-u:\b)";

macro_rules! generate_regex_matcher {
    ($regex:expr) => {
        /// Try to return a string slice corresponding to this matcher's regex.
        fn try_regex_match<'a>(&self, haystack: &'a str) -> Option<&'a str> {
            static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new($regex).unwrap());
            REGEX
                .captures(haystack)
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str())
        }
    };
}

/// Implementers of this trait are capable of attempting to match [`Token`]s from preprocessed C
/// source code.
pub trait TokenMatcher {
    /// Try to match a [`Token`] at the start of the input.
    /// Return [`None`] if unsuccessful.
    fn try_match<'a>(&self, input: &'a str, position: usize) -> Option<Token<'a>>;
}

/// Matches C keywords (`int`, `void`, `return`, etc.).
pub struct KeywordMatcher;
impl KeywordMatcher {
    generate_regex_matcher!(KEYWORD_REGEX);
}
impl TokenMatcher for KeywordMatcher {
    fn try_match<'a>(&self, input: &'a str, position: usize) -> Option<Token<'a>> {
        let matched = self.try_regex_match(input)?;
        let keyword = Keyword::try_from(matched).ok()?;
        Some(Token::new(TokenKind::Keyword(keyword), position))
    }
}

/// Matches C identifiers (variable names, function names, etc.).
pub struct IdentifierMatcher;
impl IdentifierMatcher {
    generate_regex_matcher!(IDENTIFIER_REGEX);
}
impl TokenMatcher for IdentifierMatcher {
    fn try_match<'a>(&self, input: &'a str, position: usize) -> Option<Token<'a>> {
        let matched = self.try_regex_match(input)?;
        Some(Token::new(TokenKind::Identifier(matched), position))
    }
}

/// Matches integer constants.
pub struct ConstantMatcher;
impl ConstantMatcher {
    generate_regex_matcher!(CONSTANT_REGEX);
}
impl TokenMatcher for ConstantMatcher {
    fn try_match<'a>(&self, input: &'a str, position: usize) -> Option<Token<'a>> {
        let matched = self.try_regex_match(input)?;
        Some(Token::new(TokenKind::Constant(matched), position))
    }
}

/// Matches single-character symbols (parentheses, braces, semicolons, etc.).
pub struct SymbolMatcher;
impl TokenMatcher for SymbolMatcher {
    fn try_match<'a>(&self, input: &'a str, position: usize) -> Option<Token<'a>> {
        let first_char = input.chars().next()?;
        for symbol in Symbol::iter() {
            if first_char == symbol.into() {
                return Some(Token::new(TokenKind::Symbol(symbol), position));
            }
        }

        None
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn keyword_matcher_int() {
        let matcher = KeywordMatcher;
        let token = matcher.try_match("int main", 0).unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(Keyword::Int));
        assert_eq!(token.len(), 3);
    }

    #[test]
    fn keyword_matcher_not_keyword() {
        let matcher = KeywordMatcher;
        assert!(matcher.try_match("inti main", 0).is_none());
    }

    #[test]
    fn identifier_matcher_simple() {
        let matcher = IdentifierMatcher;
        let token = matcher.try_match("my_var = 5;", 0).unwrap();
        assert_eq!(token.kind, TokenKind::Identifier("my_var"));
        assert_eq!(token.len(), 6);
    }

    #[test]
    fn identifier_matcher_not_identifier() {
        let matcher = IdentifierMatcher;
        assert!(matcher.try_match("1hi", 0).is_none());
    }

    #[test]
    fn identifier_matcher_non_ascii() {
        let matcher = IdentifierMatcher;
        assert!(matcher.try_match("你好吗 = \"good\";", 0).is_none());
    }

    #[test]
    fn identifier_matcher_non_ascii_end() {
        let matcher = IdentifierMatcher;
        let token = matcher.try_match("test_1234五", 0).unwrap();
        assert_eq!(token.kind, TokenKind::Identifier("test_1234"));
        assert_eq!(token.len(), 9);
    }

    #[test]
    fn constant_matcher_number() {
        let matcher = ConstantMatcher;
        let token = matcher.try_match("12345;", 0).unwrap();
        assert_eq!(token.kind, TokenKind::Constant("12345"));
        assert_eq!(token.len(), 5);
    }

    #[test]
    fn constant_matcher_not_constant() {
        let matcher = ConstantMatcher;
        assert!(matcher.try_match("{", 0).is_none());
    }

    #[test]
    fn symbol_matcher_semicolon() {
        let matcher = SymbolMatcher;
        let token = matcher.try_match(";", 10).unwrap();
        assert_eq!(token.kind, TokenKind::Symbol(Symbol::Semicolon));
        assert_eq!(token.span.start, 10);
    }

    #[test]
    fn symbol_matcher_no_match() {
        let matcher = SymbolMatcher;
        assert!(matcher.try_match("@", 0).is_none());
    }
}
